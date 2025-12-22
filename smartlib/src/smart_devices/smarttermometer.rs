use std::fmt;

#[derive(Debug, Clone)]
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

/// Реализация умного термометра
/// Возможно переключение различных мер измерений, при этом температура будет конвертироваться
#[derive(Debug, Clone)]
pub struct SmartThermometer {
    name: String,
    tempreture: f32,
    measure: TempMeasures,
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Термометр '{}', Температура: {}{}",
            self.get_name(),
            self.get_tempreture(),
            self.measure
        )
    }
}

impl SmartThermometer {
    pub fn new(name: String, measure: TempMeasures, tempreture: f32) -> Self {
        Self {
            name,
            measure,
            tempreture,
        }
    }

    pub fn change_measure(&mut self) {
        match self.measure {
            TempMeasures::C => {
                self.measure = TempMeasures::F;
                self.tempreture = (self.tempreture * 9.0 / 5.0) + 32.0
            }
            TempMeasures::F => {
                self.measure = TempMeasures::C;
                self.tempreture = (self.tempreture - 32.0) * 5.0 / 9.0
            }
        }
    }

    pub fn get_tempreture(&self) -> f32 {
        self.tempreture
    }

    pub fn get_measure(&self) -> &str {
        match self.measure {
            TempMeasures::F => "F",
            TempMeasures::C => "C",
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
