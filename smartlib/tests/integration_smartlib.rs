use smartlib::structures::Report;
use smartlib::{Room, SmartDevice, SmartHome};

#[test]
fn reports_include_room_and_device_lines() {
    // build room with two devices and verify report structure contains lines
    let mut room = Room::new(String::from("Гостиная"));

    let thermo = smartlib::smart_devices::SmartThermometer::new(
        String::from("Termo1"),
        smartlib::smart_devices::TempMeasures::C,
        21.5,
    );
    let socket = smartlib::smart_devices::SmartElectricalSoket::new(String::from("Sock1"), 150.0);

    room.add_device_with_key(String::from("Termo1"), thermo.into());
    room.add_device_with_key(String::from("Sock1"), SmartDevice::ElectricalSocket(socket));

    let mut home = SmartHome::new(String::from("MyHome"), vec![room]);

    // also ensure add/get work in public API from integration
    home.add_room_with_key(String::from("Кухня"), Room::new(String::from("Кухня")));

    let report = home.report();

    assert!(report.contains("Отчет для дома: MyHome"));
    assert!(report.contains("Комната 'Гостиная'"));
    assert!(report.contains("| -- "));
}

#[test]
fn errors_are_displayed_meaningfully() {
    let home = SmartHome::new(String::from("H"), vec![]);
    let err = home.get_device_from_room("Nope", "X").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Room Nope not found"));
}
