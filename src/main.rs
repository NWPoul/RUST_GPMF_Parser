use std::time::Duration;

use rusb::{Context, Device, DeviceDescriptor, Result, UsbContext};

fn print_device_info<T: UsbContext>(device: Device<T>, descriptor: DeviceDescriptor) -> Result<()> {
    let bus_number = device.bus_number();
    let address = device.address();
    println!(
        "Bus {:03} Device {:03} ID {:04x}:{:04x}",
        bus_number,
        address,
        descriptor.vendor_id(),
        descriptor.product_id()
    );

    match device.open() {
        Ok(handle) => {
            if let Ok(language) = handle.read_languages(Duration::new(1, 0)) {
                if !language.is_empty() {
                    let language = language[0];
                    
                    if let Ok(manufacturer) = handle.read_manufacturer_string(language, &descriptor, Duration::new(1, 0)) {
                        println!("  Manufacturer: {}", manufacturer);
                    }
                    
                    if let Ok(product) = handle.read_product_string(language, &descriptor, Duration::new(1, 0)) {
                        println!("  Product: {}", product);
                    }
                    
                    if let Ok(serial_number) = handle.read_serial_number_string(language, &descriptor, Duration::new(1, 0)) {
                        println!("  Serial Number: {}", serial_number);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("  Unable to open device: {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let context = Context::new()?;
    for device in context.devices()?.iter() {
        let descriptor = device.device_descriptor()?;
        print_device_info(device, descriptor)?;
    }
    Ok(())
}


// use sysinfo::Disks;

// pub fn get_current_drives2() -> (Disks, Vec<String>) {
//     let mut drivers_list: Vec<String> = vec![];
//     let disks = Disks::new_with_refreshed_list();
//     for disk in disks.list() {
//         println!("{:?}: {:?}", disk.name(), disk.kind());
//         drivers_list.push(
//             format!("{:?}", disk)
//         )

//     }
//     (disks, drivers_list)
// }