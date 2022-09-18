use std::fmt;

pub enum Device {
    Socket(SmartSocket),
    Thermo(SmartThermometer),
}
pub struct SmartSocket {
    on: bool,
    load: u32,
}

pub struct SmartThermometer {
    temperature: f32,
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

    pub fn set_load(&mut self, load: u32) {
        self.load = load;
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket::new()
    }
}

impl Device {
    pub fn self_info(&self) -> String {
        match self {
            Device::Socket(socket) => {
                let state = if socket.on { "ON" } else { "OFF" };
                format!(
                    "device_info: [SmartSocket] state: {}. load: {}",
                    state, socket.load
                )
            }
            Device::Thermo(thermo) => {
                format!(
                    "device_info: [SmartThermometer] temperature: {}",
                    thermo.temperature
                )
            }
        }
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
