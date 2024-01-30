use crate::devices::DeviceInfoProvider;

pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

pub struct Room {
    name: String,
    devices: Vec<String>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: vec![
                Room {
                    name: "Living Room".to_string(),
                    devices: vec!["TV".to_string(), "Lamp".to_string(), "Thermo".to_string()],
                },
                Room {
                    name: "Kitchen".to_string(),
                    devices: vec![
                        "Lamp".to_string(),
                        "Thermo".to_string(),
                        "Fridge".to_string(),
                    ],
                },
            ],
        }
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name.as_str()).collect()
    }

    pub fn devices(&self, room_name: &str) -> Vec<&str> {
        self.rooms
            .iter()
            .find(|r| r.name == room_name)
            .map_or(Vec::new(), |r| {
                r.devices.iter().map(AsRef::as_ref).collect()
            })
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = String::new();
        report.push_str(self.name.as_str());
        for room in &self.rooms {
            for device in &room.devices {
                let info = provider.get_device_status(&room.name, device);
                report.push_str(&format!("{}\n", info));
            }
        }
        report
    }
}
