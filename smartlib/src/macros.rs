#[macro_export]
macro_rules! add_room {
    ($room_name: expr) => {{
        $crate::structures::Room::new($room_name)
    }};
    ($room_name: expr, ($key: expr, $device: expr)) => {{
        let mut room = $crate::structures::Room::new($room_name);
        room.add_device_with_key(String::from($key), $device.into());
        room
    }};
    ($room_name: expr, $(($key: expr, $device: expr)),+ $(,)?) => {{
        let mut room = $crate::structures::Room::new($room_name);
        $(
            room.add_device_with_key(String::from($key), $device.into());
        )+
        room
    }};
}
