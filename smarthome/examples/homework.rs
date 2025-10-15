use smartlib::{
    SmartHome, add_room,
    errors::SmartHomeErrors,
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
    structures::Report,
};

/// Функция, которая печает отчет по выбранному объекту
fn print_report<T: Report>(reported_obj: &T) {
    println!("{}", reported_obj.report())
}

/// Пример использования библиотеки для умного доме
/// В данном примере показан базовый подход к работе с умным домом.
fn main() {
    // Устройства для Гостинной
    let new_termometer =
        SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C, 24.0);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), 220.0);

    // Устройства для Кухни
    let kitchen_termometer =
        SmartThermometer::new(String::from("KitchenTermo"), TempMeasures::C, 25.0);
    let kitchen_socket1 = SmartElectricalSoket::new(String::from("SocketForMicrowave"), 220.0);
    let mut kitchen_socket2 = SmartElectricalSoket::new(String::from("SocketFreezer"), 220.0);
    kitchen_socket2.turn_on();

    // Создаем кухню с помощью макроса
    let room = add_room!(
        String::from("Гостинная"),
        ("T", new_termometer),
        ("S1", some_electrical_soket)
    );

    // Макрос допускает создание пустой комнаты
    let mut room2 = add_room!(String::from("Кухня"));

    // Динамическое добавление устройств в комнату.
    room2.add_device_with_key("S1".to_string(), kitchen_socket1.into());
    room2.add_device_with_key("S2".to_string(), kitchen_socket2.into());
    room2.add_device_with_key("T1".to_string(), kitchen_termometer.into());

    let mut smart_home = SmartHome::new("MyHome".to_string(), vec![room]);

    // Динамическое добавление комнаты в дом
    smart_home.add_room_with_key("Кухня".to_string(), room2);

    // Возможность вызова отчетов для Дома, Комнаты и умного устройства
    print_report(&smart_home);

    let reported_room = match smart_home.get_room("Кухня") {
        Some(rp) => {
            print_report(rp);
            rp
        }
        None => panic!("Room not found"),
    };

    match reported_room.get_device("S1") {
        Some(dv) => {
            print_report(dv);
        }
        None => panic!("Device not found"),
    };

    // Обработка ошибок
    match smart_home.get_device_from_room("Кухня", "T2") {
        Ok(dv) => {
            println!("{}", dv.report())
        }
        Err(SmartHomeErrors::RoomNotFound(room)) => {
            eprintln!("❌ : комната '{}' не найдена", room);
        }
        Err(SmartHomeErrors::DeviceNotFound(device)) => {
            eprintln!("❌: устройство '{}' не найдено в указанной комнате", device);
        }
    }
    // Удаление комнаты
    match smart_home.delete_room("Столовая") {
        Ok(()) => println!("✅ Комната успешно удалена из дома"),
        Err(SmartHomeErrors::RoomNotFound(room)) => eprintln!("❌ : Комната {} не найдена", room),
        Err(e) => eprintln!("❌: Неизвестная ошибка: {}", e),
    };

    print_report(&smart_home);

    // Удаление устройства
    let room_for_delete = smart_home
        .get_mutable_room("Кухня")
        .expect("❌: No room Found");
    println!("Отчет перед удалением умного устройтва");
    print_report(room_for_delete);
    match room_for_delete.delete_device("S1") {
        Ok(()) => println!("✅ Устройство успешно удалено из комнаты"),
        Err(SmartHomeErrors::DeviceNotFound(device)) => {
            eprintln!("❌ : Устройство {} не найдено", device)
        }
        Err(e) => eprintln!("❌: Неизвестная ошибка: {}", e),
    }
    print_report(room_for_delete);
}
