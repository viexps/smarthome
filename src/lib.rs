use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Write},
};

pub struct SmartSocket {
    on: bool,
    load: u32,
}

pub struct SmartThermometer {
    temperature: f32,
}

pub trait Device {
    fn self_info(&self) -> String;
}

pub struct Room {
    pub devices: HashMap<String, Box<dyn Device>>,
}

pub struct SmartHouse {
    pub rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> HashSet<&String> {
        self.rooms.keys().collect()
    }

    pub fn device_names(&self, room: &str) -> HashSet<&String> {
        if let Some(room) = self.rooms.get(room) {
            room.devices.keys().collect()
        } else {
            HashSet::new()
        }
    }

    pub fn create_report(&self, info_provider: &dyn DeviceInfoProvider) -> String {
        let mut report = String::from("---SmartHouse---\n");
        for (room_name, room) in self.rooms.iter() {
            writeln!(&mut report, "room: {}", room_name).unwrap();

            for (device_name, device) in room.devices.iter() {
                writeln!(&mut report, "device: {}", device_name).unwrap();
                writeln!(
                    &mut report,
                    "{}",
                    &info_provider.device_info(room_name, device_name, device.as_ref())
                )
                .unwrap();
            }
        }

        report
    }
}

impl Default for SmartHouse {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device_name: &str, device: &dyn Device) -> String;
}

pub struct DefaultDeviceInfoProvider {}

impl DeviceInfoProvider for DefaultDeviceInfoProvider {
    fn device_info(&self, _room: &str, _device_name: &str, device: &dyn Device) -> String {
        device.self_info()
    }
}

impl SmartSocket {
    pub fn new() -> Self {
        Self { on: false, load: 0 }
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
        self.load = 0;
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket::new()
    }
}

impl Device for SmartSocket {
    fn self_info(&self) -> String {
        let state = if self.on { "ON" } else { "OFF" };
        format!(
            "device_info: [SmartSocket] state: {}. load: {}",
            state, self.load
        )
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.on {
            write!(f, "[socket] state: on. load: {}", self.load)
        } else {
            write!(f, "[socket] state: off")
        }
    }
}

impl SmartThermometer {
    pub fn new() -> Self {
        Self { temperature: 0.0 }
    }
}

impl Default for SmartThermometer {
    fn default() -> Self {
        SmartThermometer::new()
    }
}

impl Device for SmartThermometer {
    fn self_info(&self) -> String {
        format!(
            "device_info: [SmartThermometer] temperature: {}",
            self.temperature
        )
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[thermometer] temperature: {}", self.temperature)
    }
}

#[cfg(test)]
mod tests {
    use crate::{SmartSocket, SmartThermometer};

    #[test]
    fn basic_socket() {
        let mut s = SmartSocket::new();
        assert_eq!(s.to_string(), "[socket] state: off");

        s.turn_on();
        assert_eq!(s.on, true);
        assert_eq!(s.to_string(), "[socket] state: on. load: 0");
    }

    #[test]
    fn basic_thermometer() {
        let s = SmartThermometer::new();
        assert_eq!(s.temperature, 0.0)
    }
}
