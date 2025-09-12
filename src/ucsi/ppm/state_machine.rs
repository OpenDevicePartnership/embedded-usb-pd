//! This module defines the core PPM state machine as defined in 6.1 of the UCSI spec.
//! <https://www.usb.org/document-library/usb-type-cr-connector-system-software-interface-ucsi-specification>
//!
//! The state machine presented in the spec is a combination of state (circular boxes) as well as
//! actions (rectangular boxes) that should be done when transitioning between states. How to perform these actions
//! is left to the implementation. The state machine presented here abstracts over these actions with the [`Input`] enum.
//! The [`Output`] enum defines the notifications to the OPM.

use crate::{ucsi, GlobalPortId, LocalPortId, PortId};

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
    /// Waiting for async event ack
    WaitForAsyncEventAck,
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
    /// There's a pending async event
    PendingAsyncEvent,
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
    /// Notify that ack was received
    OpmNotifyAckComplete,
    /// Notify OPM of async event
    OpmNotifyAsyncEvent,
    /// PPM reset complete
    ResetComplete,
    /// Notify OPM that PPM is busy
    OpmNotifyBusy,
}

pub type GlobalOutput<'a> = Output<'a, GlobalPortId>;
pub type LocalOutput<'a> = Output<'a, LocalPortId>;

/// Attempted transition that is not allowed by the state machine
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTransition<'a, T: PortId> {
    /// The current state of the state machine
    pub state: State,
    /// The input that was attempted
    pub input: Input<'a, T>,
}

// Doctest tries to compile the mermaid code as rust so just disable it
#[cfg_attr(not(doctest), aquamarine::aquamarine)]
#[cfg_attr(not(doctest), doc = "include_mmd!(\"docs/ucsi/ppm_state_machine.mmd\")")]
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
            // Idle(false) transitions
            (Idle(false), Command(cmd @ ucsi::Command::PpmCommand(ucsi::ppm::Command::SetNotificationEnable(_)))) => {
                (ProcessingCommand, Some(ExecuteCommand(cmd)))
            }
            (Idle(false), BusyChanged) => (Busy(false), None),

            // Busy transitions
            (Busy(notification_enabled), BusyChanged) => (Idle(notification_enabled), None),
            (Busy(false), CommandComplete) => (Busy(false), None),
            (Busy(true), CommandComplete) => (Busy(true), Some(OpmNotifyBusy)),

            // Idle(true) transitions
            (Idle(true), BusyChanged) => (Busy(true), None),
            (Idle(true), PendingAsyncEvent) => (WaitForAsyncEventAck, Some(OpmNotifyAsyncEvent)),
            (Idle(true), Command(cmd)) => (ProcessingCommand, Some(ExecuteCommand(cmd))),

            // ProcessingCommand transitions
            (ProcessingCommand, CommandComplete | Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::Cancel))) => {
                (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete))
            }

            // WaitForCommandCompleteAck transitions
            (WaitForCommandCompleteAck, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args)))) => {
                if args.ack.command_complete() {
                    (Idle(true), Some(OpmNotifyAckComplete))
                } else {
                    return Err(InvalidTransition {
                        state: self.state,
                        input,
                    });
                }
            }

            // WaitForAsyncEventAck transitions
            (WaitForAsyncEventAck, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::AckCcCi(args)))) => {
                if args.ack.connector_change() {
                    (Idle(true), None)
                } else {
                    return Err(InvalidTransition {
                        state: self.state,
                        input,
                    });
                }
            }

            // Reset transitions
            (_, Command(ucsi::Command::PpmCommand(ucsi::ppm::Command::PpmReset))) => (Idle(false), Some(ResetComplete)),

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
