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

fn main3() -> Result<()> {
    let context = Context::new()?;
    for device in context.devices()?.iter() {
        let descriptor = device.device_descriptor()?;
        print_device_info(device, descriptor)?;
    }
    Ok(())
}

    use std::process::Command;

fn main() {
    let directory_path = r"D:\DEV\VIDEO_TEMP\PARSED_VIDEO"; // Replace with your actual folder path

    // Command to open the folder in Windows Explorer
    let open_folder_command = format!("explorer \"{}\"", directory_path);
    let _ = Command::new("powershell")
                    .arg("-Command")
                    .arg(open_folder_command)
                    .output()
                    .expect("Failed to execute command");

    // Command to find the most recently modified file in the folder
    let select_latest_file_command = format!(
        "Get-ChildItem -Path \"{}\" | Sort-Object LastWriteTime -Descending | Select-Object -First 1",
        directory_path,
    );
    let output = Command::new("powershell")
                       .arg("-Command")
                       .arg(select_latest_file_command)
                       .output()
                       .expect("Failed to execute command");

    // Assuming the latest file name is printed to stdout
    let latest_file_name = String::from_utf8_lossy(&output.stdout);
    println!("Latest file: {}", latest_file_name.trim());

    // Note: This code does not actually select the file in Windows Explorer; it merely prints the name of the latest file.
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