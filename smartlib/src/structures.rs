use std::fmt;

use rand::{Rng, rng};

pub trait GetStatus {
    fn print_status(&mut self) -> String;
}

pub enum TempMeasures {
    C,
    F,
}

impl fmt::Display for TempMeasures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TempMeasures::C => write!(f, "° C"),
            TempMeasures::F => write!(f, "° F"),
        }
    }
}

pub struct SmartThermometer {
    name: String,
    temp: i8,
    measure: TempMeasures,
}

impl SmartThermometer {
    pub fn new(name: String, measure: TempMeasures) -> Self {
        let temp = rng().random();
        Self {
            name,
            temp,
            measure,
        }
    }

    fn get_new_temp(&mut self) {
        self.temp = rng().random();
    }
}

impl GetStatus for SmartThermometer {
    fn print_status(&mut self) -> String {
        self.get_new_temp();
        format!(
            "Device Name: {}, Current Temp: {}{}",
            self.name, self.temp, self.measure
        )
    }
}

pub struct SmartElectricalSoket {
    switch_status: bool,
    power: i32,
}

pub struct SmartDevice<T: GetStatus> {
    device: T,
}

impl<T: GetStatus> SmartDevice<T> {
    fn new(device: T) -> Self {
        Self { device }
    }
    fn get_device_status(&mut self) {
        println!("{}", self.device.print_status())
    }
}
