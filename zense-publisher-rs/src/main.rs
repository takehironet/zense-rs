use zenseapi::enums::{DataMode, DepthRange, FrameType, PropertyType, PropertyValue, ZenseError};

fn main() {
    print!("initialize...");
    let ret = zenseapi::initialize();
    println!("done");
    let _ = dbg!(ret);
    print!("get_device_count...");
    let ret = zenseapi::get_device_count();
    let count = match ret {
        Ok(n) => {
            println!("{}", n);
            n
        }
        Err(e) => {
            dbg!(e);
            0
        }
    };
    let ret = zenseapi::get_device_list_info(count).unwrap();
    dbg!(ret);
    let ret = zenseapi::get_device_info(0).unwrap();
    dbg!(ret);

    print!("open_device...");
    let mut handle = zenseapi::open_device("/dev/video0").unwrap();
    println!("opened");

    let session_index: u32 = 0;

    for _x in 0..100 {
        let prop = handle.get_property(0, PropertyType::SerialNumber);
        if let Ok(v) = prop {
            match v {
                PropertyValue::StringValue(s) => {
                    let s = String::from_utf8(s.as_bytes().to_vec()).unwrap();
                    println!("{}", s);
                }
                PropertyValue::Uint8Value(_) => {}
                PropertyValue::Int32ValueList(_) => {}
            }
        };

        print!("start_stream...");
        let stream_start = std::time::Instant::now();
        let _ = handle.start_stream(session_index);
        println!("started");

        let _ = handle.get_data_mode(session_index);
        let _ = handle.set_data_mode(session_index, DataMode::WdrDepth);

        let threshold = handle.get_threshold(session_index);
        dbg!(threshold);

        for _x in 0..30 {
            let start = std::time::Instant::now();
            //
            // let _depth_range = handle.get_depth_range(session_index);
            // let _res = handle.set_depth_range(session_index, DepthRange::FarRange);
            // let _depth_range = handle.get_depth_range(session_index);

            let _ = handle.read_next_frame(session_index);
            let dur_read_next_frame = start.elapsed();
            let start = std::time::Instant::now();
            let _ = handle.get_frame(session_index, FrameType::DepthFrame);
            let dur_get_frame = start.elapsed();
            println!(
                "ReadNextFrame: {} ns, GetFrame: {} ns",
                dur_read_next_frame.as_nanos(),
                dur_get_frame.as_nanos()
            );
        }
        println!("stream stop");
        let _ = handle.stop_stream(session_index);
        let dur_stream = stream_start.elapsed();
        dur_stream.as_millis();
        println!("Stream duration: {} milli seconds", dur_stream.as_millis());
    }

    println!("closing device");
    let _res = handle.close_device();
    println!("closed");
    let ret = zenseapi::shutdown();
    let _ = dbg!(ret);
}
