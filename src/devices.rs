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
