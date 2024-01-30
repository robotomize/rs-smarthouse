pub trait DeviceInfoProvider {
    fn get_device_status(&self, room: &str, device: &str) -> String;
}

pub struct SmartSocket {}

impl DeviceInfoProvider for SmartSocket {
    fn get_device_status(&self, _: &str, device: &str) -> String {
        format!(
            "Info: {}",
            match device {
                "TV" => "State: On",
                "Lamp" => "Luminosity: 70%",
                "Fridge" => "220w",
                _ => "Unknown device",
            }
        )
    }
}

pub struct SmartThermometer {}

impl DeviceInfoProvider for SmartThermometer {
    fn get_device_status(&self, _: &str, device: &str) -> String {
        format!(
            "Info: {}",
            match device {
                "Thermo" => "Temp: 20C",
                _ => "Unknown device",
            }
        )
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_status(&self, room: &str, device: &str) -> String {
        format!(
            "Room: {}, Device: {}, {}",
            room,
            device,
            match room {
                "Living Room" => match device {
                    "TV" | "Lamp" | "Fridge" => self.socket.get_device_status(room, device),
                    _ => "Unknown device".to_string(),
                },
                "Kitchen" => match device {
                    "TV" | "Lamp" | "Fridge" => self.socket.get_device_status(room, device),
                    _ => "Unknown device".to_string(),
                },
                _ => "Unknown room".to_string(),
            }
        )
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_status(&self, room: &str, device: &str) -> String {
        format!(
            "Room: {}, Device: {}, {}",
            room,
            device,
            match room {
                "Living Room" => match device {
                    "TV" | "Lamp" | "Fridge" => self.socket.get_device_status(room, device),
                    "Thermo" => self.thermo.get_device_status(room, device),
                    _ => "Unknown device".to_string(),
                },
                "Kitchen" => match device {
                    "TV" | "Lamp" | "Fridge" => self.socket.get_device_status(room, device),
                    "Thermo" => self.thermo.get_device_status(room, device),
                    _ => "Unknown device".to_string(),
                },
                _ => "Unknown room".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod smart_socket_tests {
    use super::*;

    #[test]
    fn test_smart_socket_tv() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status("", "TV"), "Info: State: On");
    }

    #[test]
    fn test_smart_socket_lamp() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status("", "Lamp"), "Info: Luminosity: 70%");
    }

    #[test]
    fn test_smart_socket_fridge() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status("", "Fridge"), "Info: 220w");
    }

    #[test]
    fn test_smart_socket_unknown() {
        let socket = SmartSocket {};
        assert_eq!(socket.get_device_status("", "Super lamp"), "Info: Unknown device");
    }
}

#[cfg(test)]
mod smart_thermometer_tests {
    use super::*;

    #[test]
    fn test_smart_thermometer_thermo() {
        let thermo = SmartThermometer {};
        assert_eq!(thermo.get_device_status("", "Thermo"), "Info: Temp: 20C");
    }

    #[test]
    fn test_smart_thermometer_unknown() {
        let thermo = SmartThermometer {};
        assert_eq!(thermo.get_device_status("", "My therm"), "Info: Unknown device");
    }
}

#[cfg(test)]
mod owning_device_info_provider_tests {
    use super::*;

    #[test]
    fn test_owning_provider_tv() {
        let socket = SmartSocket {};
        let provider = OwningDeviceInfoProvider { socket };
        assert_eq!(provider.get_device_status("Living Room", "TV"), "Room: Living Room, Device: TV, Info: State: On");
    }

    #[test]
    fn test_owning_provider_room_unknown() {
        let socket = SmartSocket {};
        let provider = OwningDeviceInfoProvider { socket };
        assert_eq!(provider.get_device_status("Child room", "TV"), "Room: Child room, Device: TV, Unknown room");
    }
}

#[cfg(test)]
mod borrowing_device_info_provider_tests {
    use super::*;

    #[test]
    fn test_borrowing_provider_thermo() {
        let socket = SmartSocket {};
        let thermo = SmartThermometer {};
        let provider = BorrowingDeviceInfoProvider { socket: &socket, thermo: &thermo };
        assert_eq!(provider.get_device_status("Kitchen", "Thermo"), "Room: Kitchen, Device: Thermo, Info: Temp: 20C");
    }

    #[test]
    fn test_borrowing_provider_unknown() {
        let socket = SmartSocket {};
        let thermo = SmartThermometer {};
        let provider = BorrowingDeviceInfoProvider { socket: &socket, thermo: &thermo };
        assert_eq!(provider.get_device_status("Kitchen new", "Thermo"), "Room: Kitchen new, Device: Thermo, Unknown room");
    }
}