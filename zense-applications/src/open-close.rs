use zenseapi as zense;

fn main() {
    zense::initialize().expect("Failed to initialize zense");
    let device_count = zense::get_device_count().expect("Failed to get device count");
    match device_count {
        n if n > 1 => println!("{} devices found", device_count),
        n if n == 1 => println!("1 device found"),
        _ => println!("No device found"),
    };
    for i in 0..device_count {
        let device_info = zense::get_device_info(i).expect("Failed to get device info");
        eprintln!("Device: {}", i);
        for i in 1..=20 {
            eprint!("Attempt: {:>2} -> ", i);
            if let Ok(mut handle) = zense::open_device(device_info.uri.to_str().unwrap()) {
                if handle.close_device().is_ok() {
                    eprintln!("OK!");
                } else {
                    eprintln!("Failed (CloseDevice)!");
                }
            } else {
                eprintln!("Failed (OpenDevice)!");
            }
        }
        eprintln!();
    }
    let _ = zense::shutdown();
}
