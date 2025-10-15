pub mod errors;
pub mod macros;
pub mod smart_devices;
pub mod structures;
pub use crate::structures::{Room, SmartDevice, SmartHome};

#[cfg(test)]
mod tests;
