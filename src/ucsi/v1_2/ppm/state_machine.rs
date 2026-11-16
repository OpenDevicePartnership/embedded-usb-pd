//! This module defines the core PPM state machine as defined in 6.1 of the UCSI spec.
//! <https://www.usb.org/document-library/usb-type-cr-connector-system-software-interface-ucsi-specification>
//!
//! The state machine presented in the spec is a combination of state (circular boxes) as well as
//! actions (rectangular boxes) that should be done when transitioning between states. How to perform these actions
//! is left to the implementation. The state machine presented here abstracts over these actions with the [`Input`] enum.
//! The [`Output`] enum defines the notifications to the OPM.

use crate::ucsi::v1_2 as ucsi;
use crate::ucsi::v1_2::ppm::ack_cc_ci;
use crate::{GlobalPortId, LocalPortId, PortId};

/// PPM states
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    /// Idle
    Idle,
    /// Busy
    Busy,
    /// Processing Command,
    ProcessingCommand,
    /// Waiting for command complete ack
    WaitForCommandCompleteAck,
}

/// Inputs to the PPM state machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Input<'a, T: PortId> {
    /// UCSI command
    Command(&'a ucsi::Command<T>),
    /// Command completed successfully,
    CommandComplete,
    /// Command failed with an error
    CommandError,
    /// External busy status changed
    BusyChanged,
}

pub type GlobalInput<'a> = Input<'a, GlobalPortId>;
pub type LocalInput<'a> = Input<'a, LocalPortId>;

/// Outputs from the PPM state machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Output<'a, T: PortId> {
    /// Execute the command
    ExecuteCommand(&'a ucsi::Command<T>),
    /// Notify OPM that command completed
    OpmNotifyCommandComplete,
    /// Ack completed
    AckComplete(ack_cc_ci::Ack),
    /// PPM reset complete
    ResetComplete,
    /// Notify OPM that PPM is busy
    OpmNotifyBusy,
    /// Notify OPM that command failed with an error
    OpmNotifyCommandError,
}

pub type GlobalOutput<'a> = Output<'a, GlobalPortId>;
pub type LocalOutput<'a> = Output<'a, LocalPortId>;

/// Attempted transition that is not allowed by the state machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTransition<'a, T: PortId> {
    /// The current state of the state machine
    pub state: State,
    /// Whether notifications are enabled
    pub notification_enabled: bool,
    /// The input that was attempted
    pub input: Input<'a, T>,
}

/// PPM State Machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine<T: PortId> {
    state: State,
    notification_enabled: bool,
    _marker: core::marker::PhantomData<T>,
}

impl<T: PortId> StateMachine<T> {
    /// Create a new state machine in the Idle(false) state
    pub const fn new() -> Self {
        StateMachine {
            state: State::Idle,
            notification_enabled: false,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the current state
    pub fn state(&self) -> State {
        self.state
    }

    /// Returns whether notification is enabled
    pub fn notification_enabled(&self) -> bool {
        self.notification_enabled
    }

    /// Transition the state machine based on the input and return the output to the OPM if any.
    pub fn consume<'a>(&mut self, input: Input<'a, T>) -> Result<Option<Output<'a, T>>, InvalidTransition<'a, T>> {
        use Input::*;
        use Output::*;
        use State::*;

        let (next_state, output) = match (self.state, input) {
            // Reset transitions
            (_, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::PpmReset))) => {
                self.notification_enabled = false;
                (Idle, Some(ResetComplete))
            }

            // Idle, notification enable status independent transitions
            (Idle, BusyChanged) => (Busy, None),

            // Idle, notification disabled, transitions
            (Idle, Command(cmd @ ucsi::Command::PpmCommand(ucsi::ppm::Command::SetNotificationEnable(_))))
                if !self.notification_enabled =>
            {
                (ProcessingCommand, Some(ExecuteCommand(cmd)))
            }

            // Busy transitions
            (Busy, BusyChanged) => (Idle, None),
            (Busy, CommandComplete) => (Busy, Some(OpmNotifyBusy)),
            (Busy, Command(_cmd)) => (Busy, Some(OpmNotifyBusy)),

            // Idle, notification enabled, successful transitions
            (Idle, Command(cmd @ ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args))))
                if self.notification_enabled =>
            {
                if args.ack.command_complete {
                    // This should only happen in WaitForCommandCompleteAck
                    return Err(InvalidTransition {
                        state: self.state,
                        notification_enabled: self.notification_enabled,
                        input,
                    });
                } else {
                    (ProcessingCommand, Some(ExecuteCommand(cmd)))
                }
            }
            (Idle, Command(cmd)) if self.notification_enabled => (ProcessingCommand, Some(ExecuteCommand(cmd))),

            // ProcessingCommand transitions
            (ProcessingCommand, CommandComplete | Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::Cancel))) => {
                // If we're here, notification have to be enabled because this is either the command complete for a set notification,
                // or it's for a different command which means that set notification was already executed
                self.notification_enabled = true;
                (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete))
            }
            (ProcessingCommand, CommandError) => (Idle, Some(OpmNotifyCommandError)),

            // WaitForCommandCompleteAck transitions
            (WaitForCommandCompleteAck, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args))))
                if args.ack.command_complete =>
            {
                (Idle, Some(AckComplete(args.ack)))
            }

            // Invalid transition
            _ => {
                return Err(InvalidTransition {
                    state: self.state,
                    notification_enabled: self.notification_enabled,
                    input,
                })
            }
        };

        self.state = next_state;
        Ok(output)
    }
}

impl<T: PortId> Default for StateMachine<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub type GlobalStateMachine = StateMachine<GlobalPortId>;
pub type LocalStateMachine = StateMachine<LocalPortId>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::ucsi::v1_2::{lpm, ppm, Command};

    #[test]
    fn test_reset_all() {
        let mut sm = GlobalStateMachine::new();

        // Test reset from all states
        sm.state = State::Idle;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Test reset from Idle(true)
        sm.state = State::Idle;
        sm.notification_enabled = true;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Test reset from Busy(false)
        sm.state = State::Busy;
        sm.notification_enabled = false;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Test reset from Busy(true)
        sm.state = State::Busy;
        sm.notification_enabled = true;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Test reset from ProcessingCommand
        sm.state = State::ProcessingCommand;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Test reset from WaitForCommandCompleteAck
        sm.state = State::WaitForCommandCompleteAck;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);
    }

    /// Test that only SET_NOTIFICATION_ENABLE works from Idle when notification is disabled
    #[test]
    fn test_idle_notification_disabled_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::Idle;
        sm.notification_enabled = false;

        // Valid command
        let cmd = Command::PpmCommand(ppm::Command::SetNotificationEnable(
            ucsi::ppm::set_notification_enable::Args::default(),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::ExecuteCommand(&cmd))));
        assert_eq!(sm.state(), State::ProcessingCommand);
        assert_eq!(sm.notification_enabled, false);

        let res = sm.consume(Input::CommandComplete);
        assert_eq!(res, Ok(Some(Output::OpmNotifyCommandComplete)));
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck);
        assert_eq!(sm.notification_enabled, true);

        // Invalid PPM command
        sm.state = State::Idle;
        sm.notification_enabled = false;
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ucsi::ppm::ack_cc_ci::Args::default()));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle,
                notification_enabled: false,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Invalid LPM command
        sm.state = State::Idle;
        sm.notification_enabled = false;
        let cmd = Command::LpmCommand(ucsi::lpm::Command::new(lpm::CommandData::GetPdos(
            lpm::get_pdos::Args::default(),
        )));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle,
                notification_enabled: false,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);
    }

    /// Test that cancel works while processing a command, but no other commands are accepted
    #[test]
    fn test_processing_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::ProcessingCommand;
        sm.notification_enabled = true;

        // State machine is already in processing command state, should fail
        let cmd = Command::LpmCommand(lpm::Command::new(lpm::CommandData::GetPdos(
            lpm::get_pdos::Args::default(),
        )));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::ProcessingCommand,
                notification_enabled: true,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::ProcessingCommand);

        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::Cancel)));
        assert_eq!(res, Ok(Some(Output::OpmNotifyCommandComplete)));
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck)
    }

    /// Test that a command failing with an error transitions correctly when notification is enabled
    #[test]
    fn test_processing_command_error_notification_enabled() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::ProcessingCommand;
        sm.notification_enabled = true;

        let res = sm.consume(Input::CommandError);
        assert_eq!(res, Ok(Some(Output::OpmNotifyCommandError)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, true);

        // Should not be valid from the idle state
        let res = sm.consume(Input::CommandError);
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle,
                notification_enabled: true,
                input: Input::CommandError
            })
        );
    }

    /// Test that a command failing with an error transitions correctly when notification is disabled
    #[test]
    fn test_processing_command_error_notification_disabled() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::ProcessingCommand;
        sm.notification_enabled = false;

        let res = sm.consume(Input::CommandError);
        assert_eq!(res, Ok(Some(Output::OpmNotifyCommandError)));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, false);

        // Should not be valid from the idle state
        let res = sm.consume(Input::CommandError);
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle,
                notification_enabled: false,
                input: Input::CommandError
            })
        );
    }

    /// Test idle command transitions with notification enabled
    #[test]
    fn test_idle_notification_enabled_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::Idle;
        sm.notification_enabled = true;

        // Test simple command execution
        let cmd = Command::LpmCommand(lpm::Command::new(lpm::CommandData::GetPdos(
            lpm::get_pdos::Args::default(),
        )));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::ExecuteCommand(&cmd))));
        assert_eq!(sm.state(), State::ProcessingCommand);

        // Test rejection of command completion ACK
        sm.state = State::Idle;
        sm.notification_enabled = true;
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: ppm::ack_cc_ci::Ack {
                command_complete: true,
                ..Default::default()
            },
        }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle,
                notification_enabled: true,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, true);

        // Test acceptance of connector change ACK
        sm.state = State::Idle;
        sm.notification_enabled = true;
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: ppm::ack_cc_ci::Ack {
                connector_change: true,
                ..Default::default()
            },
        }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::ExecuteCommand(&cmd))));
        assert_eq!(sm.state(), State::ProcessingCommand);
    }

    /// Test wait for command complete command transitions
    #[test]
    fn test_wait_for_command_complete_ack_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::WaitForCommandCompleteAck;
        sm.notification_enabled = true;

        // Command complete ACK should succeed
        let ack = ppm::ack_cc_ci::Ack {
            command_complete: true,
            ..Default::default()
        };
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args { ack }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::AckComplete(ack))));
        assert_eq!(sm.state(), State::Idle);
        assert_eq!(sm.notification_enabled, true);

        // Connector change ACK only should fail
        sm.state = State::WaitForCommandCompleteAck;
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: ppm::ack_cc_ci::Ack {
                connector_change: true,
                ..Default::default()
            },
        }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::WaitForCommandCompleteAck,
                notification_enabled: true,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck);

        // All other commands should fail as well
        sm.state = State::WaitForCommandCompleteAck;
        let cmd = Command::LpmCommand(lpm::Command::new(lpm::CommandData::GetPdos(
            lpm::get_pdos::Args::default(),
        )));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::WaitForCommandCompleteAck,
                notification_enabled: true,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck);
    }

    /// Test response when attempting to execute a command when busy, with notification enabled
    #[test]
    fn test_busy_notify() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::Busy;
        sm.notification_enabled = true;

        let result = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::GetCapability)));
        assert_eq!(result, Ok(Some(Output::OpmNotifyBusy)));
        assert_eq!(sm.state(), State::Busy);
        assert_eq!(sm.notification_enabled, true);
    }
}
