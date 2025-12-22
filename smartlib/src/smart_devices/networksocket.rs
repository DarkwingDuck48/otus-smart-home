use crate::network::{DeviceCommands, TcpDevice};
use std::error::Error;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocketCommand {
    /// Включить розетку
    TurnOn,
    /// Выключить розетку
    TurnOff,
    /// Переключить состояние
    Switch,
    /// Получить текущую мощность
    GetPower,
    /// Получить статус
    GetStatus,
}

/// Реализация умного термометра с возможностью Tcp подключения
#[derive(Debug)]
pub struct TCPSmartElectricalSocket {
    name: String,
    address: String,
    power: f32,
    is_on: bool,
    stream: Option<TcpStream>,
}

impl fmt::Display for TCPSmartElectricalSocket {
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

impl DeviceCommands for TCPSmartElectricalSocket {
    type CommandType = SocketCommand;

    fn command_to_string(&self, command: &Self::CommandType) -> &'static str {
        match command {
            SocketCommand::TurnOn => "ON",
            SocketCommand::TurnOff => "OFF",
            SocketCommand::Switch => "SWITCH",
            SocketCommand::GetPower => "GET_POWER",
            SocketCommand::GetStatus => "GET_STATUS",
        }
    }

    fn parse_response(
        &self,
        command: &Self::CommandType,
        response: &str,
    ) -> Result<String, String> {
        match command {
            SocketCommand::TurnOn | SocketCommand::TurnOff | SocketCommand::Switch => {
                if response.starts_with("OK:") {
                    Ok(response.to_string())
                } else {
                    Err(format!("Некорректный ответ: {}", response))
                }
            }
            SocketCommand::GetPower => {
                if response.starts_with("POWER:") {
                    Ok(response.to_string())
                } else {
                    Err(format!("Некорректный ответ для GET_POWER: {}", response))
                }
            }
            SocketCommand::GetStatus => {
                if response.starts_with("STATUS:") {
                    Ok(response.to_string())
                } else {
                    Err(format!("Некорректный ответ для GET_STATUS: {}", response))
                }
            }
        }
    }

    fn available_commands(&self) -> Vec<Self::CommandType> {
        vec![
            SocketCommand::TurnOn,
            SocketCommand::TurnOff,
            SocketCommand::Switch,
            SocketCommand::GetPower,
            SocketCommand::GetStatus,
        ]
    }
}

impl TcpDevice for TCPSmartElectricalSocket {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stream.is_some() {
            return Err("Уже подключено к устройству".into());
        };
        println!("Подключаемся к {} ...", self.address);
        let stream = TcpStream::connect(&self.address)?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        self.stream = Some(stream);
        println!("Успешно подключились к {}", self.address);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.stream = None
    }

    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    fn send_command(&mut self, command: Self::CommandType) -> Result<String, Box<dyn Error>> {
        let command_str = self.command_to_string(&command);
        let stream = self.stream.as_mut().ok_or("Не подключено к устройству")?;
        stream.write_all(command_str.as_bytes())?;

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buffer[..size]).to_string();
                match self.parse_response(&command, &response) {
                    Ok(parsed_resp) => {
                        self.update_state_from_response(&command, &parsed_resp);
                        Ok(parsed_resp)
                    }
                    Err(err_msg) => Err(err_msg.into()),
                }
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => Err("Таймаут при ожидании ответа".into()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl TCPSmartElectricalSocket {
    pub fn new(name: String, power: f32, address: String) -> Self {
        Self {
            name,
            address,
            power,
            is_on: false,
            stream: None,
        }
    }

    pub fn update_state_from_response(&mut self, command: &SocketCommand, response: &str) {
        match command {
            SocketCommand::TurnOn => {
                if response == "OK:ON" {
                    self.is_on = true
                }
            }
            SocketCommand::TurnOff => {
                if response == "OK:OFF" {
                    self.is_on = false
                }
            }
            SocketCommand::Switch => {
                if response == "OK:SWITCH" {
                    self.is_on = !self.is_on
                }
            }
            SocketCommand::GetPower => {
                if response.starts_with("POWER:")
                    && let Ok(power) = response.replace("POWER:", "").trim().parse::<f32>()
                {
                    self.power = power;
                }
            }
            SocketCommand::GetStatus => {
                if response == "STATUS:ON" {
                    self.is_on = true;
                } else if response == "STATUS:OFF" {
                    self.is_on = false;
                }
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn is_on(&self) -> bool {
        self.is_on
    }
    pub fn get_power(&self) -> f32 {
        if self.is_on { self.power } else { 0.0 }
    }
    pub fn turn_on(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_command(SocketCommand::TurnOn)?;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_command(SocketCommand::TurnOff)?;
        Ok(())
    }

    pub fn switch(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_command(SocketCommand::Switch)?;
        Ok(())
    }
}
