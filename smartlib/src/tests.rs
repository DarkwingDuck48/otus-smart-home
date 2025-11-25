use crate::{
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
    structures::{Room, SmartDevice, SmartHome},
};

fn create_room() -> Room {
    let new_termometer =
        SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C, 24.0);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), 220.0);

    let another_soket =
        SmartDevice::ElectricalSocket(SmartElectricalSoket::new(String::from("Router"), 210.0));

    let mut room = Room::new(String::from("Гостинная"));
    room.add_device_with_key(String::from("RoomThermometer"), new_termometer.into());
    room.add_device_with_key(String::from("ComputerSoket"), some_electrical_soket.into());
    room.add_device_with_key(String::from("Router"), another_soket);
    room
}

fn create_home(rooms: Vec<Room>) -> SmartHome {
    SmartHome::new(String::from("TestHome"), rooms)
}

#[test]
fn test_room_get_device_option() {
    let room = create_room();
    assert!(room.get_device("RoomThermometer").is_some());
}

#[test]
fn test_mut_room_get_device_option() {
    let mut room = create_room();
    assert!(room.get_mutable_device("RoomThermometer").is_some());
}

#[test]
fn test_room_add_and_delete_device() {
    let mut room = Room::new(String::from("Bedroom"));
    // add device
    let thermo = SmartThermometer::new(String::from("T"), TempMeasures::C, 20.0);
    room.add_device_with_key(String::from("Thermo"), thermo.into());
    assert!(room.get_device("Thermo").is_some());

    // delete existing
    assert!(room.delete_device("Thermo").is_ok());
    assert!(room.get_device("Thermo").is_none());

    // delete missing
    let err = room.delete_device("Thermo").unwrap_err();
    match err {
        crate::errors::SmartHomeErrors::DeviceNotFound(name) => assert_eq!(name, "Thermo"),
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn test_macro_add_room_empty() {
    let room = crate::add_room!(String::from("Empty"));
    assert_eq!(room.get_name(), "Empty");
    assert!(room.get_device("any").is_none());
}

#[test]
fn test_macro_add_room_one_device() {
    let thermo = SmartThermometer::new(String::from("T"), TempMeasures::C, 19.0);
    let room = crate::add_room!(String::from("WithOne"), ("Thermo", thermo));
    assert!(room.get_device("Thermo").is_some());
}

#[test]
fn test_macro_add_room_multiple_devices() {
    let thermo = SmartThermometer::new(String::from("T1"), TempMeasures::C, 19.0);
    let socket = SmartElectricalSoket::new(String::from("S1"), 100.0);
    let room = crate::add_room!(
        String::from("WithMany"),
        ("Thermo1", thermo),
        ("Socket1", socket),
    );
    assert!(room.get_device("Thermo1").is_some());
    assert!(room.get_device("Socket1").is_some());
}

#[test]
fn test_home_get_device_and_errors() {
    let room = create_room();
    let home = create_home(vec![room]);

    // ok path
    let device = home
        .get_device_from_room("Гостинная", "RoomThermometer")
        .unwrap();
    match device {
        SmartDevice::Thermometer(_) => {}
        _ => panic!("unexpected device type"),
    }

    // missing device
    let err = home
        .get_device_from_room("Гостинная", "Unknown")
        .unwrap_err();
    match err {
        crate::errors::SmartHomeErrors::DeviceNotFound(name) => assert_eq!(name, "Unknown"),
        _ => panic!("unexpected error variant"),
    }

    // missing room
    let err = home
        .get_device_from_room("Кухня", "RoomThermometer")
        .unwrap_err();
    match err {
        crate::errors::SmartHomeErrors::RoomNotFound(name) => assert_eq!(name, "Кухня"),
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn test_home_add_and_delete_room() {
    let mut home = create_home(vec![]);
    let room = Room::new(String::from("Балкон"));
    home.add_room_with_key(String::from("Балкон"), room);
    assert!(home.get_room("Балкон").is_some());

    assert!(home.delete_room("Балкон").is_ok());
    assert!(home.get_room("Балкон").is_none());

    let err = home.delete_room("Балкон").unwrap_err();
    match err {
        crate::errors::SmartHomeErrors::RoomNotFound(name) => assert_eq!(name, "Балкон"),
        _ => panic!("unexpected error variant"),
    }
}
