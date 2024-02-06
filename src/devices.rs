use std::fmt;

pub trait DeviceInfoProvider {
    fn get_device_status(&self, room: &RoomTypes, device: &DeviceTypes) -> Option<String>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceTypes {
    TV,
    Lamp,
    Fridge,
    Thermo,
}

impl fmt::Display for DeviceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct SmartSocket {}

impl DeviceInfoProvider for SmartSocket {
    fn get_device_status(&self, _: &RoomTypes, device: &DeviceTypes) -> Option<String> {
        match device {
            DeviceTypes::TV => Some("State: On".to_string()),
            DeviceTypes::Lamp => Some("Luminosity: 70%".to_string()),
            DeviceTypes::Fridge => Some("220w".to_string()),
            _ => None,
        }
    }
}

pub struct SmartThermometer {}

impl DeviceInfoProvider for SmartThermometer {
    fn get_device_status(&self, _: &RoomTypes, device: &DeviceTypes) -> Option<String> {
        match device {
            DeviceTypes::Thermo => Some("Temp: 20C".to_string()),
            _ => None,
        }
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RoomTypes {
    Living,
    Kitchen,
}

impl fmt::Display for RoomTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_status(&self, room: &RoomTypes, device: &DeviceTypes) -> Option<String> {
        match room {
            RoomTypes::Living => match device {
                DeviceTypes::TV | DeviceTypes::Lamp | DeviceTypes::Fridge => self.socket.get_device_status(room, device),
                _ => None,
            },
            RoomTypes::Kitchen => match device {
                DeviceTypes::TV | DeviceTypes::Lamp | DeviceTypes::Fridge => self.socket.get_device_status(room, device),
                _ => None,
            },
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_status(&self, room: &RoomTypes, device: &DeviceTypes) -> Option<String> {
        match room {
            RoomTypes::Living => match device {
                DeviceTypes::TV | DeviceTypes::Lamp | DeviceTypes::Fridge => self.socket.get_device_status(room, device),
                DeviceTypes::Thermo => self.thermo.get_device_status(room, device),
            },
            RoomTypes::Kitchen => match device {
                DeviceTypes::TV | DeviceTypes::Lamp | DeviceTypes::Fridge => self.socket.get_device_status(room, device),
                DeviceTypes::Thermo => self.thermo.get_device_status(room, device),
            },
        }
    }
}

#[cfg(test)]
mod smart_socket_tests {
    use crate::devices::DeviceTypes::{Fridge, Lamp, TV};
    use super::*;

    #[test]
    fn test_smart_socket_tv() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status(&RoomTypes::Living, &TV), Some("State: On".to_string()));
    }

    #[test]
    fn test_smart_socket_lamp() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status(&RoomTypes::Living, &Lamp), Some("Luminosity: 70%".to_string()));
    }

    #[test]
    fn test_smart_socket_fridge() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status(&RoomTypes::Living, &Fridge), Some("220w".to_string()));
    }
}

#[cfg(test)]
mod smart_thermometer_tests {
    use crate::devices::DeviceTypes::Thermo;
    use super::*;

    #[test]
    fn test_smart_thermometer_thermo() {
        let thermo = SmartThermometer {};
        assert_eq!(thermo.get_device_status(&RoomTypes::Living, &Thermo), Some("Temp: 20C".to_string()));
    }

    #[test]
    fn test_smart_thermometer_unknown() {
        let thermo = SmartThermometer {};
        assert_eq!(thermo.get_device_status(&RoomTypes::Living, &DeviceTypes::TV), None);
    }
}

#[cfg(test)]
mod owning_device_info_provider_tests {
    use crate::devices::DeviceTypes::TV;
    use crate::devices::RoomTypes::Living;
    use super::*;

    #[test]
    fn test_owning_provider_tv() {
        let socket = SmartSocket {};
        let provider = OwningDeviceInfoProvider { socket };
        assert_eq!(provider.get_device_status(&Living, &TV), Some("State: On".to_string()));
    }
}

#[cfg(test)]
mod borrowing_device_info_provider_tests {
    use crate::devices::DeviceTypes::Thermo;
    use crate::devices::RoomTypes::Kitchen;
    use super::*;

    #[test]
    fn test_borrowing_provider_thermo() {
        let socket = SmartSocket {};
        let thermo = SmartThermometer {};
        let provider = BorrowingDeviceInfoProvider { socket: &socket, thermo: &thermo };
        assert_eq!(provider.get_device_status(&Kitchen, &Thermo), Some("Temp: 20C".to_string()));
    }
}