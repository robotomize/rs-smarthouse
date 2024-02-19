mod devices;
mod house;

use crate::devices::{BorrowingDeviceInfoProvider, DeviceTypes, OwningDeviceInfoProvider, SmartSocket, SmartThermometer};
use crate::house::SmartHouse;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let mut h = SmartHouse::new("my house");

    h.add_device("Kitchen", DeviceTypes::TV);
    h.remove_device("Kitchen", DeviceTypes::TV);
    print!("Rooms with bathroom:\n");
    h.add_room("BathRoom");
    for room in h.get_rooms() {
        println!("Room: {}", room)
    }
    h.remove_room("BathRoom");
    println!("\nRooms without bathroom:");
    for room in h.get_rooms() {
        println!("Room: {}", room)
    }
    h.remove_room("BathRoom");
    println!("\nHouse specification:");
    for room in h.get_rooms() {
        if let Some(devices) = h.devices(&room) {
            for device in devices {
                println!("Room: {}, Devices: {:?}", room, device);
            }
        }
    }

    println!();

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = h.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = h.create_report(&info_provider_2);

    println!("Report #1:\n{}", report1);
    println!("Report #2:\n{}", report2);
}
