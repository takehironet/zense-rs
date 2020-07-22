use std::process::exit;

use zenseapi::enums::{PropertyType, PropertyValue, SensorType};

fn main() {
    if let Err(e) = zenseapi::initialize() {
        eprintln!("Initialize Error: {:?}", e);
        exit(1);
    };

    let device_count = match zenseapi::get_device_count() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Get device count error: {:?}", e);
            let _ = zenseapi::shutdown();
            exit(1);
        }
    };

    let device_list_info = match zenseapi::get_device_list_info(device_count) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Get device list info error: {:?}", e);
            let _ = zenseapi::shutdown();
            exit(1);
        }
    };

    let session_index: u32 = 0;
    device_list_info.iter().for_each(|info| {
        let uri = info.uri.to_str();
        if let Ok(s) = uri {
            let mut handle = match zenseapi::open_device(s) {
                Ok(handle) => handle,
                Err(_e) => return,
            };
            let serial_number = match handle.get_property(session_index, PropertyType::SerialNumber)
            {
                Ok(PropertyValue::StringValue(s)) => {
                    s.into_string().unwrap_or_else(|_| String::from(""))
                }
                _ => String::from(""),
            };
            if let Ok(p) = handle.get_camera_parameters(session_index, SensorType::DepthSensor) {
                println!("[{}_Factory]", serial_number);
                println!("fx = {}", p.fx);
                println!("fy = {}", p.fy);
                println!("cx = {}", p.cx);
                println!("cy = {}", p.cy);
                println!("k1 = {}", p.k1);
                println!("k2 = {}", p.k2);
                println!("p1 = {}", p.p1);
                println!("p2 = {}", p.p2);
                println!("k3 = {}", p.k3);
                println!("k4 = {}", p.k4);
                println!("k5 = {}", p.k5);
                println!("k6 = {}", p.k6);
                println!();
            };
            let _ = handle.close_device();
        }
    });
    let _ = zenseapi::shutdown();
}
