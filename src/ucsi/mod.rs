// UCSI v1.2 implementation
#[cfg(feature = "ucsi-v1_2")]
pub mod v1_2;

use crate::PortId;

/// Conversion of a structure between port ID types
///
/// Structures that carry port IDs (connector numbers, connector change indicators, etc.) are generic over the port
/// ID type. This trait provides a uniform way to re-target them to a different port ID type.
pub trait MapPort<T: PortId, Out: PortId>: Sized {
    /// This structure re-targeted to the port ID type `Out`
    type Output;

    /// Converts this structure to a different port type by applying `f` to each port ID it contains
    fn map_port<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> Out;

    /// Converts this structure to a different port type using the `From` trait for each port ID it contains
    fn map_port_into(self) -> Self::Output
    where
        Out: From<T>,
    {
        self.map_port(Out::from)
    }
}

/// Commands that target a connector
pub trait ConnectorNumber<T: PortId> {
    /// Returns the connector this command targets
    fn connector_number(&self) -> T;

    /// Sets the connector this command targets
    fn set_connector_number(&mut self, connector: T) -> &mut Self;
}
