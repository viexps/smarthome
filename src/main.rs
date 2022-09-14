use std::collections::HashMap;

use smarthome::{
    DefaultDeviceInfoProvider, Device, Room, SmartHouse, SmartSocket, SmartThermometer,
};

fn main() {
    let socket1 = SmartSocket::new();
    let socket2 = SmartSocket::new();
    let thermo = SmartThermometer::new();

    let room1 = {
        let s1: Box<dyn Device> = Box::new(socket1);

        Room {
            devices: HashMap::from([("socket1".to_string(), s1)]),
        }
    };

    let room2 = {
        let s2: Box<dyn Device> = Box::new(socket2);
        let t: Box<dyn Device> = Box::new(thermo);

        Room {
            devices: HashMap::from([("socket2".to_string(), s2), ("thermo".to_string(), t)]),
        }
    };

    let sh = SmartHouse {
        rooms: HashMap::from([("room1".to_string(), room1), ("room2".to_string(), room2)]),
    };

    let rooms = sh.get_rooms();
    println!("SmartHouse.get_rooms: {:?}", rooms);

    let room_devices = sh.device_names("room2");
    println!("SmartHouse.device_names: {:?}", room_devices);

    let report = sh.create_report(&DefaultDeviceInfoProvider {});
    println!("{}", report)
}
