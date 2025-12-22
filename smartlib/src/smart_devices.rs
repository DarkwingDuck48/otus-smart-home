mod networksocket;
mod smartsocket;
mod smarttermometer;

pub use networksocket::{SocketCommand, TCPSmartElectricalSocket};
pub use smartsocket::SmartElectricalSoket;
pub use smarttermometer::{SmartThermometer, TempMeasures};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::DeviceCommands;
    use crate::network::TcpDevice;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};
    use std::thread;

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

    // тесты для розетки с TCP
    #[test]
    fn test_socket_command_enum() {
        let socket =
            TCPSmartElectricalSocket::new("Test".to_string(), 220.0, "127.0.0.1:8080".to_string());

        // Проверяем преобразование команд в строки
        assert_eq!(socket.command_to_string(&SocketCommand::TurnOn), "ON");
        assert_eq!(socket.command_to_string(&SocketCommand::TurnOff), "OFF");
        assert_eq!(socket.command_to_string(&SocketCommand::Switch), "SWITCH");
        assert_eq!(
            socket.command_to_string(&SocketCommand::GetPower),
            "GET_POWER"
        );
        assert_eq!(
            socket.command_to_string(&SocketCommand::GetStatus),
            "GET_STATUS"
        );

        // Проверяем список команд
        let commands = socket.available_commands();
        assert_eq!(commands.len(), 5);
        assert!(commands.contains(&SocketCommand::TurnOn));
        assert!(commands.contains(&SocketCommand::GetStatus));
    }

    #[test]
    fn test_socket_command_parsing() {
        let socket =
            TCPSmartElectricalSocket::new("Test".to_string(), 220.0, "127.0.0.1:8080".to_string());

        // Проверяем парсинг ответов
        assert!(
            socket
                .parse_response(&SocketCommand::TurnOn, "OK:ON")
                .is_ok()
        );
        assert!(
            socket
                .parse_response(&SocketCommand::TurnOn, "ERROR")
                .is_err()
        );

        assert!(
            socket
                .parse_response(&SocketCommand::GetPower, "POWER:220.0")
                .is_ok()
        );
        assert!(
            socket
                .parse_response(&SocketCommand::GetPower, "220.0")
                .is_err()
        );
    }

    #[test]
    fn test_network_socket_creation() {
        let socket = TCPSmartElectricalSocket::new(
            "TestSocket".to_string(),
            220.0,
            "127.0.0.1:9999".to_string(),
        );

        assert_eq!(socket.get_name(), "TestSocket");
        assert!(!socket.is_on());
        assert!(!socket.is_connected());
        assert_eq!(socket.get_power(), 0.0);
    }

    #[test]
    fn test_socket_connection_simple() -> Result<(), Box<dyn std::error::Error>> {
        // Используем порт 0 для автоматического выбора свободного порта
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;

        // Настраиваем неблокирующий режим для accept
        listener.set_nonblocking(true)?;

        // Создаем сокет ДО запуска сервера
        let mut socket = TCPSmartElectricalSocket::new(
            "TestSocket".to_string(),
            220.0,
            format!("127.0.0.1:{}", addr.port()),
        );

        // Запускаем сервер в отдельном потоке
        let server_handle = std::thread::spawn(move || {
            // Ждем соединения
            for _ in 0..10 {
                // 10 попыток
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        // Обрабатываем одно соединение
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(size) => {
                                let command = String::from_utf8_lossy(&buffer[..size]);
                                stream.write_all("OK:ON".as_bytes()).unwrap();
                                println!("Сервер обработал: {}", command);
                                break;
                            }
                            Err(_) => break,
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Нет соединений, ждем
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Ошибка accept: {}", e);
                        break;
                    }
                }
            }
        });

        // Даем время серверу начать слушать
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Подключаемся
        socket.connect()?;
        assert!(socket.is_connected());

        // Отправляем команду
        let result = socket.send_command(SocketCommand::TurnOn);

        // Ждем сервер (но не бесконечно)
        if server_handle.join().is_err() {
            eprintln!("Внимание: сервер не завершился вовремя");
        }

        // Проверяем результат
        match result {
            Ok(response) => {
                assert_eq!(response, "OK:ON");
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    #[test]
    fn test_socket_connection_error() {
        let mut socket = TCPSmartElectricalSocket::new(
            "Test".to_string(),
            220.0,
            "127.0.0.1:99999".to_string(), // Неверный порт
        );

        // Попытка подключения к несуществующему порту должна вернуть ошибку
        assert!(socket.connect().is_err());
        assert!(!socket.is_connected());
    }

    #[test]
    fn test_socket_helper_methods() -> Result<(), Box<dyn std::error::Error>> {
        use std::sync::{Arc, Barrier};

        // Барьер для синхронизации клиента и сервера
        let barrier = Arc::new(Barrier::new(2));
        let barrier_clone = Arc::clone(&barrier);

        // Запускаем TCP сервер
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;

        let server_handle = std::thread::spawn(move || {
            // Делаем сокет блокирующим
            listener.set_nonblocking(false).unwrap();

            // Ждем сигнала от клиента
            barrier_clone.wait();

            // Принимаем соединение с таймаутом
            let (mut stream, _) = match listener.accept() {
                Ok((stream, addr)) => {
                    println!("Сервер: принято соединение от {}", addr);
                    (stream, addr)
                }
                Err(e) => {
                    eprintln!("Сервер: ошибка accept: {}", e);
                    return;
                }
            };

            // Устанавливаем таймауты
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(1)))
                .unwrap();
            stream
                .set_write_timeout(Some(std::time::Duration::from_secs(1)))
                .unwrap();

            // Обрабатываем команды в цикле
            for expected_cmd in &["ON", "OFF"] {
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(size) if size > 0 => {
                        let command = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
                        println!(
                            "Сервер: получил команду '{}', ожидал '{}'",
                            command, expected_cmd
                        );

                        // Проверяем что команда совпадает с ожидаемой
                        if command == *expected_cmd {
                            let response = match command.as_str() {
                                "ON" => "OK:ON",
                                "GET_POWER" => "POWER:220.0",
                                "OFF" => "OK:OFF",
                                _ => "ERROR",
                            };

                            println!("Сервер: отправляю ответ '{}'", response);
                            if let Err(e) = stream.write_all(response.as_bytes()) {
                                eprintln!("Сервер: ошибка отправки: {}", e);
                                break;
                            }
                        } else {
                            eprintln!("Сервер: неожиданная команда: {}", command);
                            break;
                        }
                    }
                    Ok(0) => {
                        println!("Сервер: клиент отключился");
                        break;
                    }
                    Ok(_) => {
                        // size == 0 уже обработано выше
                    }
                    Err(e) => {
                        eprintln!("Сервер: ошибка чтения: {}", e);
                        break;
                    }
                }

                // Небольшая пауза между командами
                std::thread::sleep(std::time::Duration::from_millis(50));
            }

            println!("Сервер: завершаю работу");
        });

        // Даем время серверу запуститься
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Создаем клиентский сокет
        let mut socket = TCPSmartElectricalSocket::new(
            "TestSocket".to_string(),
            220.0,
            format!("127.0.0.1:{}", addr.port()),
        );

        // Сигнализируем серверу
        println!("Клиент: сигнализирую серверу");
        barrier.wait();

        // Подключаемся
        println!("Клиент: подключаюсь...");
        socket.connect()?;
        assert!(socket.is_connected());

        // Тестируем методы с задержками между вызовами
        println!("Клиент: тестирую turn_on()");
        socket.turn_on()?;
        assert!(socket.is_on());
        std::thread::sleep(std::time::Duration::from_millis(100));

        // println!("Клиент: тестирую update_power()");
        // let power = socket.update_power()?;
        // assert_eq!(power, 220.0);
        // std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Клиент: тестирую turn_off()");
        socket.turn_off()?;
        assert!(!socket.is_on());

        // Даем время серверу завершиться
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Ждем сервер
        if let Err(e) = server_handle.join() {
            eprintln!("Ошибка в потоке сервера: {:?}", e);
        }

        println!("Тест завершен успешно");
        Ok(())
    }
}
