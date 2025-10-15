use crate::{
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
    structures::{Room, SmartDevice, SmartHome},
};

fn create_room() -> Room {
    let new_termometer =
        SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C, 24.0);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), 220.0);

    let termometer = SmartDevice::Thermometer(new_termometer);

    let smartsoket = SmartDevice::ElectricalSocket(some_electrical_soket);
    let another_soket =
        SmartDevice::ElectricalSocket(SmartElectricalSoket::new(String::from("Router"), 210.0));

    let mut room = Room::new(String::from("Гостинная"));
    room.add_device(termometer);
    room.add_device(smartsoket);
    room.add_device(another_soket);
    room
}

fn create_home(rooms: Vec<Room>) -> SmartHome {
    SmartHome::new(rooms)
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

// #[test]
// fn test_home_get_room_option() {
//     let rooms = create_room();
//     let home = create_home([rooms]);
//     assert!(home.get_room(1).is_none());
// }

// #[test]
// fn test_mut_home_get_room_option() {
//     let rooms = create_room();
//     let mut home = create_home([rooms]);
//     assert!(home.get_mutable_room(1).is_none());
// }
