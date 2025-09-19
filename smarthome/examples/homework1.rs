use smartlib::{
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
    {Room, SmartDevice, SmartHome},
};

/// Пример использования библиотеки для умного доме
/// В данном примере показан базовый подход к работе с умным домом.
fn main() {
    let new_termometer =
        SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C, 24.0);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), 220.0);

    let termometer = SmartDevice::Thermometer(new_termometer);

    let smartsoket = SmartDevice::ElectricalSocket(some_electrical_soket);
    let another_soket =
        SmartDevice::ElectricalSocket(SmartElectricalSoket::new(String::from("Router"), 210.0));

    let room = Room::new(
        String::from("Гостинная"),
        [smartsoket, termometer, another_soket],
    );
    let mut smart_home = SmartHome::new([room]);
    smart_home.report();
    println!("Включаем розетку в комнате...");
    if let SmartDevice::ElectricalSocket(socket) =
        smart_home.get_mutable_room(0).get_mutable_device(0)
    {
        socket.turn_on();
    }
    smart_home.report();
}
