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
