use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    ops::Deref,
};

mod device;
mod error;

pub use device::*;
pub use error::*;

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

    pub fn get_room(&self, name: impl AsRef<str>) -> Option<&Room> {
        self.rooms.get(name.as_ref())
    }

    pub fn device_names(&self, room: &str) -> HashSet<&String> {
        if let Some(room) = self.rooms.get(room) {
            room.devices.keys().collect()
        } else {
            HashSet::new()
        }
    }

    pub fn add_room(&mut self, name: impl AsRef<str>, room: Room) -> Result<&mut Self, MyError> {
        let name = name.as_ref();
        if self.rooms.contains_key(name) {
            Err(MyError::RoomAlreadyExists)
        } else {
            self.rooms.insert(name.to_owned(), room);
            Ok(self)
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

impl Room {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn device_names(&self) -> HashSet<&String> {
        self.devices.keys().collect()
    }

    pub fn get_device(&self, name: impl AsRef<str>) -> Option<&dyn Device> {
        self.devices.get(name.as_ref()).map(|x| x.deref())
    }

    pub fn add_device(&mut self, name: impl AsRef<str>, device: Box<dyn Device>) -> &mut Self {
        self.devices.insert(name.as_ref().to_owned(), device);
        self
    }
}

impl Default for Room {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let socket = SmartSocket::new();
        let thermo = SmartThermometer::new();

        let mut room = Room::new();
        room.add_device("socket", Box::new(socket))
            .add_device("thermo", Box::new(thermo));

        assert_eq!(
            room.device_names(),
            HashSet::from([&"socket".to_string(), &"thermo".to_string()])
        );

        assert!(room.get_device("socket").is_some());
    }

    #[test]
    fn test_smarthouse() -> Result<(), MyError> {
        let socket = SmartSocket::new();
        let thermo = SmartThermometer::new();

        let mut room = Room::new();
        room.add_device("socket", Box::new(socket))
            .add_device("thermo", Box::new(thermo));

        let mut sh = SmartHouse::new();
        sh.add_room("room1", room)?;

        assert_eq!(sh.get_rooms(), HashSet::from([&"room1".to_string()]));

        assert_eq!(
            sh.device_names("room1"),
            HashSet::from([&"socket".to_string(), &"thermo".to_string()])
        );
        Ok(())
    }

    #[test]
    fn add_room_with_a_same_name() -> Result<(), MyError> {
        let mut sh = SmartHouse::new();
        sh.add_room("room1", Room::new())?;
        assert_eq!(
            sh.add_room("room1", Room::new()).err(),
            Some(MyError::RoomAlreadyExists)
        );
        Ok(())
    }
}
