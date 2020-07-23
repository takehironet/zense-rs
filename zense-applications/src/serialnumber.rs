use std::{env, path::PathBuf, process::exit};

use zenseapi::{
    enums::{PropertyType, PropertyValue},
    DeviceHandle,
};

fn resolve_device_kernel_name(device_kernel_name: &str) -> Option<String> {
    let file_path_buf = PathBuf::from(format!("/dev/{}", device_kernel_name).as_str());
    match file_path_buf.canonicalize() {
        Ok(path) => match path.to_str() {
            Some(s) => Some(String::from(s)),
            None => None,
        },
        Err(_) => None,
    }
}

fn main() {
    // expects one argument, device kernel name such as "video0". it must be in /dev directory.
    // the result is PicoZense DCAM710's serial number to stdout when succeeded.
    // if this got some error, exit with non 0.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit(1);
    }
    let device_kernel_name = args.get(1).unwrap();
    let device_file_name = match resolve_device_kernel_name(device_kernel_name) {
        Some(s) => s,
        None => {
            eprintln!("Unable to resolve file path from device kernel name.");
            exit(1);
        }
    };
    zenseapi::initialize().expect("Failed to initialize API");
    let mut handle: DeviceHandle = zenseapi::open_device(device_file_name.as_str())
        .unwrap_or_else(|_| panic!("Failed to open a device {}", device_file_name));
    let serial_number = match handle.get_property(0, PropertyType::SerialNumber) {
        Ok(PropertyValue::StringValue(s)) => s.to_str().unwrap_or("").to_string(),
        _ => {
            let _ = handle.close_device();
            let _ = zenseapi::shutdown();
            exit(1);
        }
    };
    println!("{}", serial_number);
}
