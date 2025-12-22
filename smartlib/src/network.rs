use std::error::Error;

pub trait DeviceCommands {
    type CommandType;

    fn command_to_string(&self, command: &Self::CommandType) -> &'static str;
    fn parse_response(&self, command: &Self::CommandType, response: &str)
    -> Result<String, String>;
    fn available_commands(&self) -> Vec<Self::CommandType>;
}

pub trait TcpDevice: DeviceCommands {
    /// Подключение к устройству
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;

    /// Отправка проверенной команды на устройство
    fn send_command(&mut self, command: Self::CommandType) -> Result<String, Box<dyn Error>>;

    /// Отключиться от устройства
    fn disconnect(&mut self);
    /// Проверить, подключено ли устройство
    fn is_connected(&self) -> bool;
}
