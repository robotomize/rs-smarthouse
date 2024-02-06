use crate::devices::{DeviceInfoProvider, DeviceTypes, RoomTypes};
use crate::devices::DeviceTypes::{Fridge, Lamp, Thermo, TV};
use crate::devices::RoomTypes::{Kitchen, Living};

pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

pub struct Room {
    name: RoomTypes,
    devices: Vec<DeviceTypes>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: vec![
                Room {
                    name: Living,
                    devices: vec![TV, Lamp, Thermo],
                },
                Room {
                    name: Kitchen,
                    devices: vec![
                        Lamp,
                        Thermo,
                        Fridge,
                    ],
                },
            ],
        }
    }

    pub fn get_rooms(&self) -> Vec<RoomTypes> {
        self.rooms.iter().map(|r| r.name).collect()
    }

    pub fn devices(&self, room_name: &RoomTypes) -> Option<Vec<DeviceTypes>> {
        self.rooms
            .iter()
            .find(|r| r.name == *room_name)
            .map(|r| r.devices.clone())
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = String::new();
        report.push_str(format!("House: {}\n", self.name).as_str());
        for room in &self.rooms {
            for device in &room.devices {
                if let Some(status) = provider.get_device_status(&room.name, device) {
                    report.push_str(&format!("Room: {}, Device: {}, {}\n", room.name, device,
                                             status));
                }
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
    assert_eq!(rooms, vec![Living, Kitchen]);
}

#[test]
fn test_devices_in_living_room() {
    let house = SmartHouse::new("My Smart House");
    let devices = house.devices(&Living);
    assert_eq!(devices, Some(vec![TV, Lamp, Thermo]));
}

#[test]
fn test_create_report() {
    let house = SmartHouse::new("My Smart House");
    let socket1 = crate::devices::SmartSocket {};
    let info_provider_1 = crate::devices::OwningDeviceInfoProvider { socket: socket1 };
    let report = house.create_report(&info_provider_1);
    assert!(report.contains("State: On"));
    assert!(report.contains("Room: Living, Device: Lamp, Luminosity: 70%"));
}
