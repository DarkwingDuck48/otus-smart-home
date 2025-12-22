use std::fmt;

// Реализация умной розетки
/// Можно включить или выключить и посмотеть текущую мощность
#[derive(Debug, Clone)]
pub struct SmartElectricalSoket {
    name: String,
    power: f32,
    is_on: bool,
}

impl fmt::Display for SmartElectricalSoket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_on() {
            "включена"
        } else {
            "выключена"
        };
        write!(
            f,
            "Розетка '{}': {}, мощность {:.1} Вт",
            self.get_name(),
            status,
            self.get_power()
        )
    }
}

impl SmartElectricalSoket {
    pub fn new(name: String, power: f32) -> Self {
        Self {
            name,
            is_on: false,
            power,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn is_on(&self) -> bool {
        self.is_on
    }
    pub fn switch(&mut self) {
        self.is_on = !self.is_on
    }
    pub fn turn_on(&mut self) {
        self.is_on = true
    }
    pub fn turn_off(&mut self) {
        self.is_on = false
    }
    pub fn get_power(&self) -> f32 {
        match self.is_on {
            true => self.power,
            false => 0f32,
        }
    }
}
