use std::fmt;

pub struct SmartSocket {
    on: bool,
    load: u32,
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

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.on {
            write!(f, "[socket] state: on. load: {}", self.load)
        } else {
            write!(f, "[socket] state: off")
        }
    }
}

pub struct SmartThermometer {
    temperature: f32,
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
