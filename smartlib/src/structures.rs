use crate::smart_devices::{SmartElectricalSoket, SmartThermometer};

#[derive(Debug, Clone)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    ElectricalSocket(SmartElectricalSoket),
}

impl SmartDevice {
    /// Вывод инофрмации о состоянии устройства
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

pub struct Room {
    name: String,
    devices: [SmartDevice; 3],
}

impl Room {
    pub fn new(name: String, devices: [SmartDevice; 3]) -> Self {
        Self { name, devices }
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

    pub fn report(&self) {
        println!("Комната '{}': ", self.name);
        for device in self.devices.iter() {
            device.report();
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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
            room.report();
        }
    }
}
