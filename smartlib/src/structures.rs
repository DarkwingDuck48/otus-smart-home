use crate::{
    errors::SmartHomeErrors,
    smart_devices::{SmartElectricalSoket, SmartThermometer},
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    ElectricalSocket(SmartElectricalSoket),
}

impl From<SmartThermometer> for SmartDevice {
    fn from(value: SmartThermometer) -> Self {
        Self::Thermometer(value)
    }
}

impl From<SmartElectricalSoket> for SmartDevice {
    fn from(value: SmartElectricalSoket) -> Self {
        Self::ElectricalSocket(value)
    }
}

impl SmartDevice {

    fn report(&self) {
        match self {
            SmartDevice::Thermometer(thermo) => {
                println!("| -- {}", thermo)
            }
            SmartDevice::ElectricalSocket(socket) => {
                println!("| -- {}", socket)
            }
        }
    }
}

#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }

    pub fn add_device_with_key(&mut self, device_key: String, new_device: SmartDevice) {
        self.devices.insert(device_key, new_device);
    }

    pub fn delete_device(&mut self, device_name: &str) -> Result<(), String> {
        if !self.devices.contains_key(device_name) {
            return Err(format!("No device {} in the room", device_name));
        };
        self.devices.remove(device_name);
        Ok(())
    }

    pub fn get_device(&self, device_name: &str) -> Option<&SmartDevice> {
        self.devices.get(device_name)
    }

    pub fn get_mutable_device(&mut self, device_name: &str) -> Option<&mut SmartDevice> {
        self.devices.get_mut(device_name)
    }

    pub fn report(&self) {
        println!("Комната '{}': ", self.name);
        for device in self.devices.iter() {
            device.1.report();
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct SmartHome {
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(rooms: Vec<Room>) -> Self {
        let mut added_rooms = HashMap::new();
        for room in rooms {
            added_rooms.insert(room.name.clone(), room);
        }
        Self { rooms: added_rooms }
    }

    pub fn add_room_with_key(&mut self, room_key: String, new_room: Room) {
        self.rooms.insert(room_key, new_room);
    }
    pub fn get_device_from_room(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, SmartHomeErrors> {
        match self.get_room(room_name) {
            Some(room) => match room.get_device(device_name) {
                Some(device) => Ok(device),
                None => Err(SmartHomeErrors::DeviceNotFound(device_name.to_string())),
            },
            None => Err(SmartHomeErrors::RoomNotFound(room_name.to_string())),
        }
    }

    pub fn delete_room(&mut self, room_name: &str) -> Result<(), String> {
        if !self.rooms.contains_key(room_name) {
            return Err(format!("No room {} in the home", room_name));
        };
        self.rooms.remove(room_name);
        Ok(())
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }
    pub fn get_mutable_room(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    pub fn report(&self) {
        for room in &self.rooms {
            room.1.report();
        }
    }
}
