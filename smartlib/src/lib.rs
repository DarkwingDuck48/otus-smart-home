pub mod errors;
pub mod macros;
pub mod network;
pub mod smart_devices;
pub mod structures;
pub use crate::network::TcpDevice;
pub use crate::smart_devices::{SocketCommand, TCPSmartElectricalSocket};
pub use crate::structures::{Room, SmartDevice, SmartHome};

#[cfg(test)]
mod tests;
