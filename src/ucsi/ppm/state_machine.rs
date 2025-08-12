//! This module defines the core PPM state machine as defined in 6.1 of the UCSI spec.
//! <https://www.usb.org/document-library/usb-type-cr-connector-system-software-interface-ucsi-specification>
//!
//! The state machine presented in the spec is a combination of state (circular boxes) as well as
//! actions (rectangular boxes) that should be done when transitioning between states. How to perform these actions
//! is left to the implementation. The state machine presented here abstracts over these actions with the [`Input`] enum.
//! The [`Output`] enum defines the notifications to the OPM.

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
pub enum Input {
    /// Notification enabled
    NotificationEnabled,
    /// PPM reset
    Reset,
    /// Current command cancelled
    Cancel,
    /// Command completed immediately
    CommandImmediate,
    /// Command did not complete immediately
    CommandAsync,
    /// Command completed,
    CommandCompleted,
    /// Command complete ack received from OPM
    CommandCompleteAck,
    /// Async event ack received from OPM
    AsyncEventAck,
    /// External busy status changed
    BusyChanged,
    /// There's a pending async event
    PendingAsyncEvent,
}

/// Outputs from the PPM state machine
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Output {
    /// Notify OPM that command completed
    OpmNotifyCommandComplete,
    /// Notify OPM of async event
    OpmNotifyAsyncEvent,
    /// Notify OPM of PPM reset
    OpmNotifyReset,
    /// Notify OPM that PPM is busy
    OpmNotifyBusy,
}

/// Attempted transition that is not allowed by the state machine
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTransition {
    /// The current state of the state machine
    pub state: State,
    /// The input that was attempted
    pub input: Input,
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// include_mmd!("docs/ucsi/ppm_state_machine.mmd")
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine {
    state: State,
}

impl StateMachine {
    /// Create a new state machine in the Idle(false) state
    pub const fn new() -> Self {
        StateMachine {
            state: State::Idle(false),
        }
    }

    /// Returns the current state
    pub fn state(&self) -> State {
        self.state
    }

    /// Transition the state machine based on the input and return the output to the OPM if any.
    pub fn consume(&mut self, input: Input) -> Result<Option<Output>, InvalidTransition> {
        use Input::*;
        use Output::*;
        use State::*;

        let (next_state, output) = match (self.state, input) {
            // Idle(false) transitions
            (Idle(false), NotificationEnabled) => (Idle(true), None),
            (Idle(false), BusyChanged) => (Busy(false), None),
            (Idle(false), CommandImmediate | CommandAsync) => (Idle(false), None),

            // Busy transitions
            (Busy(notification_enabled), BusyChanged) => (Idle(notification_enabled), None),
            (Busy(false), CommandImmediate | CommandAsync) => (Busy(false), None),
            (Busy(true), CommandImmediate | CommandAsync) => (Busy(true), Some(OpmNotifyBusy)),

            // Idle(true) transitions
            (Idle(true), BusyChanged) => (Busy(true), None),
            (Idle(true), PendingAsyncEvent) => (WaitForAsyncEventAck, Some(OpmNotifyAsyncEvent)),
            (Idle(true), CommandImmediate) => (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete)),
            (Idle(true), CommandAsync) => (ProcessingCommand, Some(OpmNotifyBusy)),

            // ProcessingCommand transitions
            (ProcessingCommand, CommandCompleted | Cancel) => {
                (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete))
            }

            // WaitForCommandCompleteAck transitions
            (WaitForCommandCompleteAck, CommandCompleteAck) => (Idle(true), None),

            // WaitForAsyncEventAck transitions
            (WaitForAsyncEventAck, AsyncEventAck) => (Idle(true), None),
            (WaitForAsyncEventAck, CommandImmediate) => (WaitForCommandCompleteAck, Some(OpmNotifyCommandComplete)),
            (WaitForAsyncEventAck, CommandAsync) => (ProcessingCommand, Some(OpmNotifyBusy)),

            // Reset transitions
            (_, Reset) => (Idle(false), Some(OpmNotifyReset)),

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

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}
