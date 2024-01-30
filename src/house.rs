use crate::devices::{DeviceInfoProvider};

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

#[test]
fn test_house_creation() {
    let house = SmartHouse::new("My Smart House");
    assert_eq!(house.name, "My Smart House");
}

#[test]
fn test_get_rooms() {
    let house = SmartHouse::new("My Smart House");
    let rooms = house.get_rooms();
    assert_eq!(rooms, vec!["Living Room", "Kitchen"]);
}

#[test]
fn test_devices_in_living_room() {
    let house = SmartHouse::new("My Smart House");
    let devices = house.devices("Living Room");
    assert_eq!(devices, vec!["TV", "Lamp", "Thermo"]);
}

#[test]
fn test_devices_in_unknown_room() {
    let house = SmartHouse::new("My Smart House");
    let devices = house.devices("Bedroom");
    assert!(devices.is_empty());
}

#[test]
fn test_create_report() {
    let house = SmartHouse::new("My Smart House");
    let socket1 = crate::devices::SmartSocket {};
    let info_provider_1 = crate::devices::OwningDeviceInfoProvider { socket: socket1 };
    let report = house.create_report(&info_provider_1);
    assert!(report.contains(" Info: State: On"));
    assert!(report.contains("Room: Living Room, Device: Lamp, Info: Luminosity: 70%"));
}
