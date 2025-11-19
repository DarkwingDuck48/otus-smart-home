use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SmartHomeErrors {
    RoomNotFound(String),
    DeviceNotFound(String),
}

impl fmt::Display for SmartHomeErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceNotFound(device_name) => write!(f, "Device {} not found", device_name),
            Self::RoomNotFound(room_name) => write!(f, "Room {} not found", room_name),
        }
    }
}

impl Error for SmartHomeErrors {}
