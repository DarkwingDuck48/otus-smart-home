use rand::{Rng, rng};
use std::cell::Cell;
use std::fmt;

use crate::structures::GetStatus;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct SmartThermometer {
    name: String,
    measure: TempMeasures,
}

impl SmartThermometer {
    pub fn new(name: String, measure: TempMeasures) -> Self {
        Self { name, measure }
    }

    fn get_new_temp(&self) -> i8 {
        match &self.measure {
            TempMeasures::C => return rng().random_range(-30..40),
            TempMeasures::F => return rng().random_range(-86..104), // -30 to 40 degrees equvalent in F
        }
    }
}

impl GetStatus for SmartThermometer {
    fn print_status(&self) -> String {
        let current_temp = self.get_new_temp();
        format!(
            "Device Name: {}, Current Temp: {}{}",
            self.name, current_temp, self.measure
        )
    }
}
#[derive(Debug)]
pub struct SmartElectricalSoket {
    name: String,
    switch_status: Cell<bool>,
}

impl SmartElectricalSoket {
    pub fn new(name: String, switch_status: bool) -> Self {
        Self {
            name,
            switch_status: Cell::new(switch_status),
        }
    }
    pub fn switch(&self) {
        self.switch_status.set(!self.switch_status.get())
    }
    pub fn turn_on(&self) {
        self.switch_status.set(true)
    }
    pub fn turn_off(&self) {
        self.switch_status.set(false)
    }

    pub fn is_on(&self) -> bool {
        self.switch_status.get()
    }

    fn get_power(&self) -> u8 {
        match self.is_on() {
            true => return rng().random_range(1..220),
            false => 0u8,
        }
    }
}

impl GetStatus for SmartElectricalSoket {
    fn print_status(&self) -> String {
        let current_power = self.get_power();
        format!(
            "Device Name: {}, Current Power: {}, Status: {}",
            self.name,
            current_power,
            self.is_on()
        )
    }
}
