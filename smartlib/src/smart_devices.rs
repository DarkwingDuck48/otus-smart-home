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

/// Реализация умной розетки
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

#[cfg(test)]
mod tests {
    use super::*;

    // Тестируем поведение термометра
    #[test]
    fn test_new_termometer() {
        let termo = SmartThermometer::new("TestTermo".to_string(), TempMeasures::C, 32.0);
        assert_eq!(termo.get_tempreture(), 32.0);
        assert_eq!(termo.get_name(), "TestTermo");
    }

    #[test]
    fn test_termometer_change_measure_c_to_f() {
        let mut termo = SmartThermometer::new("TestTermo".to_string(), TempMeasures::C, 32.0);
        assert_eq!(termo.get_tempreture(), 32.0);
        // Из Цельсия в Фаренгейт
        termo.change_measure();
        assert_eq!(termo.get_measure(), "F");
        assert_eq!(termo.get_tempreture(), 89.6);
    }
    #[test]
    fn test_termometer_change_measure_f_to_c() {
        let mut termo = SmartThermometer::new("TestTermo".to_string(), TempMeasures::F, 89.6);
        assert_eq!(termo.get_tempreture(), 89.6);
        // Из Фаренгейта в Цельсий
        termo.change_measure();
        assert_eq!(termo.get_measure(), "C");
        assert_eq!(termo.get_tempreture(), 32.0);
    }

    #[test]
    fn test_socket() {
        let mut new_socket = SmartElectricalSoket::new(String::from("TestSocket"), 220.0);
        assert!(!new_socket.is_on(), "Expected false, but get true");
        assert_eq!(new_socket.get_power(), 0.0);
        new_socket.turn_on();
        assert_eq!(new_socket.get_power(), 220.0);
        assert!(new_socket.is_on(), "Expected true, but get false");
        new_socket.turn_off();
        assert!(!new_socket.is_on(), "Expected false, but get true");
        new_socket.switch();
        assert!(new_socket.is_on(), "Expected true, but get false");
        new_socket.switch();
        assert!(!new_socket.is_on(), "Expected false, but get true");
    }
}
