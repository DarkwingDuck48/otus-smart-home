use crate::smart_devices::{SmartElectricalSoket, SmartThermometer};

pub trait GetStatus {
    fn print_status(&self) -> String;
}

#[derive(Debug)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    ElectricalSocket(SmartElectricalSoket),
}

impl SmartDevice {
    fn report(&self) -> String {
        match self {
            SmartDevice::Thermometer(thermo) => thermo.print_status(),
            SmartDevice::ElectricalSocket(socket) => socket.print_status(),
        }
    }
}

pub struct Room {
    devices: [SmartDevice; 4],
}

impl Room {
    pub fn new(devices: [SmartDevice; 4]) -> Self {
        Self { devices }
    }

    fn check_device_index(&self, index: usize) {
        if index >= self.devices.len() {
            panic!("Device with index {} not found in Room", index)
        }
    }

    pub fn get_device(&self, index: usize) -> &SmartDevice {
        self.check_device_index(index);
        &self.devices[index]
    }

    pub fn get_mutable_device(&mut self, index: usize) -> &mut SmartDevice {
        self.check_device_index(index);
        &mut self.devices[index]
    }

    pub fn devices_report(&self) {
        for device in &self.devices {
            println!("{}", device.report());
        }
    }
}

pub struct SmartHome {
    rooms: [Room; 1],
}

impl SmartHome {
    pub fn new(rooms: [Room; 1]) -> Self {
        Self { rooms }
    }

    fn check_room_index(&self, index: usize) {
        if index >= self.rooms.len() {
            panic!("No room with index {} in SmartHome", index)
        }
    }
    pub fn get_room(&self, index: usize) -> &Room {
        self.check_room_index(index);
        &self.rooms[index]
    }
    pub fn get_mutable_room(&mut self, index: usize) -> &mut Room {
        self.check_room_index(index);
        &mut self.rooms[index]
    }

    pub fn report(&self) {
        for room in &self.rooms {
            room.devices_report();
        }
    }
}
