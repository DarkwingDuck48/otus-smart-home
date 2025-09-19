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

    Room::new(
        String::from("Гостинная"),
        [smartsoket, termometer, another_soket],
    )
}

fn create_home(rooms: [Room; 1]) -> SmartHome {
    SmartHome::new(rooms)
}

#[test]
#[should_panic]
fn test_room_panic() {
    let room = create_room();
    room.get_device(5);
}

#[test]
#[should_panic]
fn test_mut_room_panic() {
    let mut room = create_room();
    room.get_mutable_device(5);
}

#[test]
#[should_panic]
fn test_home_panic() {
    let rooms = create_room();
    let home = create_home([rooms]);
    home.get_room(1);
}

#[test]
#[should_panic]
fn test_mut_home_panic() {
    let rooms = create_room();
    let mut home = create_home([rooms]);
    home.get_mutable_room(1);
}
