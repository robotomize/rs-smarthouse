mod devices;
mod house;

use crate::devices::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartSocket, SmartThermometer,
};
use crate::house::SmartHouse;

fn main() {
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    let h = SmartHouse::new("my house");

    println!("House specification:");
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
