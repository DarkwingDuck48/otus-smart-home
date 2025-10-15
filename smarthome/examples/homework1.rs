use smartlib::{
    Room, SmartDevice, SmartHome, add_room,
    smart_devices::{SmartElectricalSoket, SmartThermometer, TempMeasures},
};

/// Пример использования библиотеки для умного доме
/// В данном примере показан базовый подход к работе с умным домом.
fn main() {
    let new_termometer =
        SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C, 24.0);
    let some_electrical_soket = SmartElectricalSoket::new(String::from("ComputerSoket"), 220.0);

    //let termometer = SmartDevice::Thermometer(new_termometer);

    //let smartsoket = SmartDevice::ElectricalSocket(some_electrical_soket);
    //let another_soket =
    //    SmartDevice::ElectricalSocket(SmartElectricalSoket::new(String::from("Router"), 210.0));

    let room = add_room!(
        String::from("BaseRoom"),
        ("T", new_termometer),
        ("S1", some_electrical_soket)
    );
    let mut smart_home = SmartHome::new(vec![room]);
    smart_home.report();
    println!("Включаем розетку в комнате...");
    if let Some(room) = smart_home.get_mutable_room("BaseRoom")
        && let Some(SmartDevice::ElectricalSocket(socket)) = room.get_mutable_device("S1")
    {
        println!("{:?}", socket.is_on());
        socket.turn_on();
    }
    smart_home.report();
}
