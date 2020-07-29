use std::str::FromStr;
use std::{env, path::PathBuf, process::exit};

use rusb::{Device, GlobalContext};
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

#[derive(Debug)]
struct UsbDeviceInfo {
    busnum: u8,
    devnum: u8,
}

fn traverse_devpath(path: PathBuf, until: PathBuf) -> Option<PathBuf> {
    if path.eq(&until) {
        return None;
    }
    let devnum_path = path.join("devnum");
    if devnum_path.as_path().exists() {
        Some(path)
    } else {
        match path.parent() {
            Some(path) => traverse_devpath(path.to_path_buf(), until),
            None => None,
        }
    }
}

fn get_busnum_devnum(devpath: &str) -> Option<UsbDeviceInfo> {
    let sys_dir = PathBuf::from("/sys");
    let devpath_dir = sys_dir.join(devpath.trim_start_matches('/'));

    if let Some(d) = traverse_devpath(devpath_dir, sys_dir) {
        let devnum = std::fs::read_to_string(d.join("devnum"));
        let busnum = std::fs::read_to_string(d.join("busnum"));

        if let (Ok(devnum), Ok(busnum)) = (devnum, busnum) {
            let devnum_busnum = (u8::from_str(devnum.trim()), u8::from_str(busnum.trim()));
            if let (Ok(devnum_u8), Ok(busnum_u8)) = devnum_busnum {
                Some(UsbDeviceInfo {
                    devnum: devnum_u8,
                    busnum: busnum_u8,
                })
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn reset_usb(usb_device_info: UsbDeviceInfo) -> rusb::Result<()> {
    let devices = rusb::devices()?;
    let matched_devices: Vec<Device<GlobalContext>> = devices
        .iter()
        .filter(|dev| {
            dev.bus_number() == usb_device_info.busnum && dev.address() == usb_device_info.devnum
        })
        .collect();
    if matched_devices.is_empty() {
        Err(rusb::Error::NoDevice)
    } else if matched_devices.len() > 1 {
        Err(rusb::Error::Other)
    } else {
        let dev = matched_devices.get(0).unwrap();
        match dev.open() {
            Ok(mut handle) => match handle.reset() {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Failed to reset device");
                    Err(e)
                }
            },
            Err(e) => Err(e),
        }
    }
}

fn main() {
    // expects one argument, device kernel name such as "video0". it must be in /dev directory.
    // the result is PicoZense DCAM710's serial number to stdout when succeeded.
    // if this got some error, exit with non 0.
    let args: Vec<String> = env::args().collect();
    if !(args.len() == 2 || args.len() == 3) {
        exit(1);
    }

    let devpath = args.get(2);
    let usb_info = match devpath {
        Some(p) => get_busnum_devnum(p.as_str()),
        None => None,
    };

    let device_kernel_name = args.get(1).unwrap();
    let device_file_name = match resolve_device_kernel_name(device_kernel_name) {
        Some(s) => s,
        None => {
            eprintln!("Unable to resolve file path from device kernel name.");
            exit(1);
        }
    };
    zenseapi::initialize().expect("Failed to initialize API");
    let mut handle: DeviceHandle = match zenseapi::open_device(device_file_name.as_str()) {
        Ok(h) => h,
        Err(_) => {
            let _ = zenseapi::shutdown();
            println!("Failed to open a device {}", device_file_name);
            match usb_info {
                Some(i) => {
                    let _ = reset_usb(i);
                    ()
                }
                None => (),
            };
            panic!()
        }
    };
    let serial_number = match handle.get_property(0, PropertyType::SerialNumber) {
        Ok(PropertyValue::StringValue(s)) => s.to_str().unwrap_or("").to_string(),
        _ => {
            let _ = handle.close_device();
            let _ = zenseapi::shutdown();
            match usb_info {
                Some(i) => {
                    let _ = reset_usb(i);
                    ()
                }
                None => (),
            };
            exit(1);
        }
    };
    println!("{}", serial_number);
}
