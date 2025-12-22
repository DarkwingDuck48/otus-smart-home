use smartlib::{SocketCommand, TCPSmartElectricalSocket, TcpDevice};

fn main() {
    let mut socket =
        TCPSmartElectricalSocket::new("Розетка".to_string(), 220.0, "127.0.0.1:8080".to_string());
    socket.connect().unwrap();

    // Типизированные команды - компилятор проверяет!
    socket.send_command(SocketCommand::TurnOn).unwrap();
    socket.send_command(SocketCommand::GetPower).unwrap();

    // Или через вспомогательные методы
    socket.turn_on().unwrap();
    socket.switch().unwrap();
}
