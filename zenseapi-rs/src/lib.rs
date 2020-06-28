use std::ffi::{CStr, CString};

use ffi::*;

use crate::enums::*;
pub use crate::structs::*;

pub mod enums;
mod ffi;
mod structs;

pub type ZenseResult<T> = Result<T, ZenseError>;

pub fn initialize() -> ZenseResult<()> {
    match unsafe { Ps2_Initialize() } {
        0 => Ok(()),
        n => Err(ZenseError::from_int(n)),
    }
}

pub fn shutdown() -> ZenseResult<()> {
    match unsafe { Ps2_Shutdown() } {
        0 => Ok(()),
        n => Err(ZenseError::from_int(n)),
    }
}

pub fn get_device_count() -> ZenseResult<u32> {
    unsafe {
        let device_count = 0;
        match Ps2_GetDeviceCount(&device_count) {
            0 => Ok(device_count),
            n => Err(ZenseError::from_int(n)),
        }
    }
}

pub fn get_device_list_info(device_count: u32) -> ZenseResult<Vec<DeviceInfo>> {
    let mut buf: Box<[PsDeviceInfo]> = vec![
        PsDeviceInfo {
            session_count: 0,
            device_type: 0,
            uri: [0; 256usize],
            fw: [0; 50usize],
            status: 0,
        };
        device_count as usize
    ]
    .into_boxed_slice();
    let data = buf.as_mut_ptr();
    match unsafe { Ps2_GetDeviceListInfo(data, device_count) } {
        0 => {
            if !data.is_null() {
                let device_info_list =
                    unsafe { std::slice::from_raw_parts_mut(data, device_count as usize) };
                Ok(device_info_list
                    .iter()
                    .map(|device_info| {
                        let uri = unsafe { CStr::from_ptr(device_info.uri.as_ptr()) }.to_owned();
                        let uri = uri.to_str().unwrap_or("").to_string();
                        let fw = unsafe { CStr::from_ptr(device_info.fw.as_ptr()) }.to_owned();
                        let fw = fw.to_str().unwrap_or("").to_string();
                        DeviceInfo {
                            session_count: device_info.session_count as i64,
                            device_type: DeviceType::from_int(device_info.device_type),
                            uri,
                            fw,
                            status: ConnectStatus::from_int(device_info.status),
                        }
                    })
                    .collect())
            } else {
                Err(ZenseError::FfiError)
            }
        }
        n => Err(ZenseError::from_int(n)),
    }
}

pub fn get_device_info(device_index: u32) -> ZenseResult<DeviceInfo> {
    let buf: *mut PsDeviceInfo = &mut PsDeviceInfo {
        session_count: 0,
        device_type: 0,
        uri: [0; 256usize],
        fw: [0; 50usize],
        status: 0,
    };

    match unsafe { Ps2_GetDeviceInfo(buf, device_index) } {
        0 if buf.is_null() => Err(ZenseError::FfiError),
        0 => {
            let uri = unsafe { CStr::from_ptr((*buf).uri.as_ptr()) }.to_owned();
            let uri = uri.to_str().unwrap_or("").to_string();
            let fw = unsafe { CStr::from_ptr((*buf).fw.as_ptr()) }.to_owned();
            let fw = fw.to_str().unwrap_or("").to_string();
            Ok(DeviceInfo {
                session_count: unsafe { *buf }.session_count as i64,
                device_type: DeviceType::from_int(unsafe { *buf }.device_type),
                uri,
                fw,
                status: ConnectStatus::from_int(unsafe { *buf }.status),
            })
        }
        n if n > 0 => Err(ZenseError::FfiError),
        n => Err(ZenseError::from_int(n)),
    }
}

pub fn open_device(uri: &str) -> ZenseResult<DeviceHandle> {
    let mut handle: PsDeviceHandle = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    let cstring_uri = CString::new(uri).unwrap();

    match unsafe { Ps2_OpenDevice(cstring_uri.as_ptr(), &mut handle) } {
        0 if handle.is_null() => Err(ZenseError::FfiError),
        0 => Ok(DeviceHandle::new(handle)),
        n if n > 0 => Err(ZenseError::FfiError),
        n => Err(ZenseError::from_int(n)),
    }
}
