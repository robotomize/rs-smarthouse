use crate::devices::{DeviceInfoProvider, DeviceTypes};
use crate::devices::DeviceTypes::{Fridge, Lamp, Thermo, TV};

pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

pub struct Room {
    name: String,
    devices: Vec<DeviceTypes>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: vec![
                Room {
                    name: "Living".to_string(),
                    devices: vec![TV, Lamp, Thermo],
                },
                Room {
                    name: "Kitchen".to_string(),
                    devices: vec![
                        Lamp,
                        Thermo,
                        Fridge,
                    ],
                },
            ],
        }
    }

    pub fn add_room(&mut self, name: &str) {
        if self.rooms.iter().find(|r| r.name == name).is_none() {
            self.rooms.push(Room {
                name: name.to_string(),
                devices: Vec::new(),
            });
        }
    }

    pub fn remove_room(&mut self, name: &str) {
        self.rooms.retain(|room| room.name != name);
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name.as_str()).collect()
    }


    pub fn add_device(&mut self, room: &str, device: DeviceTypes) {
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room) {
            room.devices.push(device);
        }
    }

    pub fn remove_device(&mut self, room: &str, device: DeviceTypes) {
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room) {
            room.devices.retain(|d| *d != device);
        }
    }

    pub fn devices(&self, room_name: &str) -> Option<Vec<DeviceTypes>> {
        self.rooms
            .iter()
            .find(|r| r.name == room_name)
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
fn test_add_room() {
    let mut house = SmartHouse { name: "".to_string(), rooms: vec![] };
    house.add_room("Living Room");
    assert_eq!(house.rooms.len(), 1);
    assert_eq!(house.rooms[0].name, "Living Room");
}

#[test]
fn test_add_room_twice() {
    let mut house = SmartHouse { name: "".to_string(), rooms: vec![] };
    house.add_room("Living Room");
    house.add_room("Living Room");
    assert_eq!(house.rooms.len(), 1); // Убедитесь, что комната добавляется только один раз
}

#[test]
fn test_remove_room() {
    let mut house = SmartHouse { name: "".to_string(), rooms: vec![Room { name: "Living Room".to_string(), devices: vec![] }] };
    house.remove_room("Living Room");
    assert!(house.rooms.is_empty());
}

#[test]
fn test_add_device() {
    let mut house = SmartHouse { name: "".to_string(), rooms: vec![Room { name: "Living Room".to_string(), devices: vec![] }] };
    house.add_device("Living Room", DeviceTypes::TV);
    assert_eq!(house.rooms[0].devices.len(), 1);
    assert_eq!(house.rooms[0].devices[0], DeviceTypes::TV);
}

#[test]
fn test_remove_device() {
    let mut house = SmartHouse { name: "".to_string(), rooms: vec![Room { name: "Living Room".to_string(), devices: vec![DeviceTypes::TV] }] };
    house.remove_device("Living Room", DeviceTypes::TV);
    assert!(house.rooms[0].devices.is_empty());
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
    assert_eq!(rooms, vec!["Living", "Kitchen"]);
}

#[test]
fn test_devices_in_living_room() {
    let house = SmartHouse::new("My Smart House");
    let devices = house.devices("Living");
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
