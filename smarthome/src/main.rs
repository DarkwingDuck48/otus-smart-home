use smartlib::{
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
    structures::{Room, SmartDevice, SmartHome},
};

fn main() {
    let new_termometer = SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C);
    let new_american_termometer =
        SmartThermometer::new(String::from("AmericanThermometer"), TempMeasures::F);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), true);

    let termometer = SmartDevice::Thermometer(new_termometer);
    let american_termometer = SmartDevice::Thermometer(new_american_termometer);
    let smartsoket = SmartDevice::ElectricalSocket(some_electrical_soket);
    let another_soket =
        SmartDevice::ElectricalSocket(SmartElectricalSoket::new(String::from("Router"), true));

    let room = Room::new([smartsoket, termometer, american_termometer, another_soket]);
    let smart_home = SmartHome::new([room]);
    smart_home.report();
}
