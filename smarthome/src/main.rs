use smartlib::structures::{GetStatus, SmartThermometer, TempMeasures};

fn main() {
    let mut new_device = SmartThermometer::new(String::from("RoomThermometer"), TempMeasures::C);
    let mut new_american_device =
        SmartThermometer::new(String::from("AmericanThermometer"), TempMeasures::F);

    println!("{}", new_device.print_status());
    println!("{}", new_american_device.print_status());
}
