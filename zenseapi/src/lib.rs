use std::os::raw::c_char;

use zenseapi_sys as raw;

use crate::enums::*;
pub use crate::structs::*;

pub mod enums;
mod structs;

pub type ZenseResult<T> = Result<T, ZenseError>;

pub fn initialize() -> ZenseResult<()> {
    match raw::initialize() {
        Ok(()) => Ok(()),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}

pub fn shutdown() -> ZenseResult<()> {
    match raw::shutdown() {
        Ok(()) => Ok(()),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}

pub fn get_device_count() -> ZenseResult<u32> {
    match raw::get_device_count() {
        Ok(n) => Ok(n),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}

fn c_char_to_string(c_chars: &[c_char]) -> String {
    c_chars
        .iter()
        .map(|&c| (c as u8) as char)
        .collect::<String>()
}

fn ps_device_info_to_device_info(ps_device_info: raw::types::PsDeviceInfo) -> DeviceInfo {
    let uri = c_char_to_string(&ps_device_info.uri);
    let fw = c_char_to_string(&ps_device_info.fw);
    DeviceInfo {
        session_count: ps_device_info.session_count as i64,
        device_type: DeviceType::from_int(ps_device_info.device_type),
        uri,
        fw,
        status: ConnectStatus::from_int(ps_device_info.status),
    }
}

pub fn get_device_list_info(device_count: u32) -> ZenseResult<Vec<DeviceInfo>> {
    match raw::get_device_list_info(device_count) {
        Ok(ps_device_info_vec) => Ok(ps_device_info_vec
            .iter()
            .map(|&ps_device_info| ps_device_info_to_device_info(ps_device_info))
            .collect::<Vec<DeviceInfo>>()),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}

pub fn get_device_info(device_index: u32) -> ZenseResult<DeviceInfo> {
    match raw::get_device_info(device_index) {
        Ok(ps_device_info) => Ok(ps_device_info_to_device_info(ps_device_info)),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}

pub fn open_device(uri: &str) -> ZenseResult<DeviceHandle> {
    match raw::open_device(uri) {
        Ok(device) => Ok(DeviceHandle::new(device)),
        Err(e) => Err(ZenseError::from_int(e)),
    }
}
