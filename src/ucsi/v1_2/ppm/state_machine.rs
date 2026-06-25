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
///
/// The spec does not explicitly distinguish between the Busy(false) and Busy(true) states. But this is done here
/// because otherwise it would allow to transition from the Idle(false) state to the Idle(true) state without
/// notifications being enabled.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    /// Idle(notification enabled)
    Idle(bool),
    /// Busy(notification enabled)
    Busy(bool),
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
    /// Command completed,
    CommandComplete,
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
}

pub type GlobalOutput<'a> = Output<'a, GlobalPortId>;
pub type LocalOutput<'a> = Output<'a, LocalPortId>;

/// Attempted transition that is not allowed by the state machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTransition<'a, T: PortId> {
    /// The current state of the state machine
    pub state: State,
    /// The input that was attempted
    pub input: Input<'a, T>,
}

/// PPM State Machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine<T: PortId> {
    state: State,
    _marker: core::marker::PhantomData<T>,
}

impl<T: PortId> StateMachine<T> {
    /// Create a new state machine in the Idle(false) state
    pub const fn new() -> Self {
        StateMachine {
            state: State::Idle(false),
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the current state
    pub fn state(&self) -> State {
        self.state
    }

    /// Transition the state machine based on the input and return the output to the OPM if any.
    pub fn consume<'a>(&mut self, input: Input<'a, T>) -> Result<Option<Output<'a, T>>, InvalidTransition<'a, T>> {
        use Input::*;
        use Output::*;
        use State::*;

        let (next_state, output) = match (self.state, input) {
            // Reset transitions
            (_, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::PpmReset))) => (Idle(false), Some(ResetComplete)),

            // Idle(false) transitions
            (Idle(false), Command(cmd @ ucsi::Command::PpmCommand(ucsi::ppm::Command::SetNotificationEnable(_)))) => {
                (ProcessingCommand, Some(ExecuteCommand(cmd)))
            }
            (Idle(false), BusyChanged) => (Busy(false), None),

            // Busy transitions
            (Busy(notification_enabled), BusyChanged) => (Idle(notification_enabled), None),
            (Busy(false), CommandComplete) => (Busy(false), None),
            (Busy(true), CommandComplete) => (Busy(true), Some(OpmNotifyBusy)),

            // Idle(true) successful transitions
            (Idle(true), BusyChanged) => (Busy(true), None),
            (Idle(true), Command(cmd @ ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args)))) => {
                if args.ack.command_complete() {
                    // This should only happen in WaitForCommandCompleteAck
                    return Err(InvalidTransition {
                        state: self.state,
                        input,
                    });
                } else {
                    (ProcessingCommand, Some(ExecuteCommand(cmd)))
                }
            }
            (Idle(true), Command(cmd)) => (ProcessingCommand, Some(ExecuteCommand(cmd))),

            // ProcessingCommand transitions
            (ProcessingCommand, CommandComplete | Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::Cancel))) => {
                (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete))
            }

            // WaitForCommandCompleteAck transitions
            (WaitForCommandCompleteAck, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args))))
                if args.ack.command_complete() =>
            {
                (Idle(true), Some(AckComplete(args.ack)))
            }

            // Invalid transition
            _ => {
                return Err(InvalidTransition {
                    state: self.state,
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
        sm.state = State::Idle(false);
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));

        // Test reset from Idle(true)
        sm.state = State::Idle(true);
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));

        // Test reset from Busy(false)
        sm.state = State::Busy(false);
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));

        // Test reset from Busy(true)
        sm.state = State::Busy(true);
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));

        // Test reset from ProcessingCommand
        sm.state = State::ProcessingCommand;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));

        // Test reset from WaitForCommandCompleteAck
        sm.state = State::WaitForCommandCompleteAck;
        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::PpmReset)));
        assert_eq!(res, Ok(Some(Output::ResetComplete)));
        assert_eq!(sm.state(), State::Idle(false));
    }

    /// Test that only SET_NOTIFICATION_ENABLE works from Idle(false)
    #[test]
    fn test_idle_false_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::Idle(false);

        // Valid command
        let cmd = Command::PpmCommand(ppm::Command::SetNotificationEnable(
            ucsi::ppm::set_notification_enable::Args::default(),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::ExecuteCommand(&cmd))));
        assert_eq!(sm.state(), State::ProcessingCommand);

        // Invalid PPM command
        sm.state = State::Idle(false);
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ucsi::ppm::ack_cc_ci::Args::default()));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle(false),
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle(false));

        // Invalid LPM command
        sm.state = State::Idle(false);
        let cmd = Command::LpmCommand(ucsi::lpm::Command::new(
            GlobalPortId(0),
            lpm::CommandData::GetPdos(lpm::get_pdos::Args::default()),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle(false),
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle(false));
    }

    /// Test that cancel works while processing a command, but no other commands are accepted
    #[test]
    fn test_processing_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::ProcessingCommand;

        // State machine is already in processing command state, should fail
        let cmd = Command::LpmCommand(lpm::Command::new(
            GlobalPortId(0),
            lpm::CommandData::GetPdos(lpm::get_pdos::Args::default()),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::ProcessingCommand,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::ProcessingCommand);

        let res = sm.consume(Input::Command(&Command::PpmCommand(ppm::Command::Cancel)));
        assert_eq!(res, Ok(Some(Output::OpmNotifyCommandComplete)));
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck)
    }

    /// Test idle true command transitions
    #[test]
    fn test_idle_true_commands() {
        let mut sm = GlobalStateMachine::new();
        sm.state = State::Idle(true);

        // Test simple command execution
        let cmd = Command::LpmCommand(lpm::Command::new(
            GlobalPortId(0),
            lpm::CommandData::GetPdos(lpm::get_pdos::Args::default()),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::ExecuteCommand(&cmd))));
        assert_eq!(sm.state(), State::ProcessingCommand);

        // Test rejection of command completion ACK
        sm.state = State::Idle(true);
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: *ppm::ack_cc_ci::Ack::default().set_command_complete(true),
        }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::Idle(true),
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::Idle(true));

        // Test acceptance of connector change ACK
        sm.state = State::Idle(true);
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: *ppm::ack_cc_ci::Ack::default().set_connector_change(true),
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

        // Command complete ACK should succeed
        let ack = *ppm::ack_cc_ci::Ack::default().set_command_complete(true);
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args { ack }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(res, Ok(Some(Output::AckComplete(ack))));
        assert_eq!(sm.state(), State::Idle(true));

        // Connector change ACK only should fail
        sm.state = State::WaitForCommandCompleteAck;
        let cmd = Command::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: *ppm::ack_cc_ci::Ack::default().set_connector_change(true),
        }));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::WaitForCommandCompleteAck,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck);

        // All other commands should fail as well
        sm.state = State::WaitForCommandCompleteAck;
        let cmd = Command::LpmCommand(lpm::Command::new(
            GlobalPortId(0),
            lpm::CommandData::GetPdos(lpm::get_pdos::Args::default()),
        ));
        let res = sm.consume(Input::Command(&cmd));
        assert_eq!(
            res,
            Err(InvalidTransition {
                state: State::WaitForCommandCompleteAck,
                input: Input::Command(&cmd)
            })
        );
        assert_eq!(sm.state(), State::WaitForCommandCompleteAck);
    }
}
