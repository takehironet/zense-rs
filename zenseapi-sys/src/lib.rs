use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::enums::*;
use crate::raw_funcs::*;
use crate::types::*;

pub mod enums;
mod raw_funcs;
pub mod types;

pub fn initialize() -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_Initialize() } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn shutdown() -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_Shutdown() } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_device_count() -> Result<u32, PsReturnStatus> {
    let mut device_count = 0;
    match unsafe { Ps2_GetDeviceCount(&mut device_count) } {
        0 => Ok(device_count),
        n => Err(n),
    }
}

pub fn get_device_list_info(device_count: u32) -> Result<Vec<PsDeviceInfo>, PsReturnStatus> {
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
                Ok(device_info_list.to_vec())
            } else {
                Err(255)
            }
        }
        n => Err(n),
    }
}

pub fn get_device_info(device_index: u32) -> Result<PsDeviceInfo, PsReturnStatus> {
    let mut info: MaybeUninit<PsDeviceInfo> = MaybeUninit::uninit();

    match unsafe { Ps2_GetDeviceInfo(info.as_mut_ptr(), device_index) } {
        0 => Ok(unsafe { info.assume_init() }),
        n => Err(n),
    }
}

pub fn open_device(uri: &str) -> Result<PsDeviceHandle, PsReturnStatus> {
    let mut handle: MaybeUninit<PsDeviceHandle> = MaybeUninit::uninit();
    let cstring_uri = CString::new(uri).unwrap();

    match unsafe { Ps2_OpenDevice(cstring_uri.as_ptr(), handle.as_mut_ptr()) } {
        0 => Ok(unsafe { handle.assume_init() }),
        n => Err(n),
    }
}

pub fn close_device(device_handle: PsDeviceHandle) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_CloseDevice(device_handle) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn start_stream(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_StartStream(device_handle, session_index) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn stop_stream(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_StopStream(device_handle, session_index) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn read_next_frame(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<PsFrameReady, PsReturnStatus> {
    // let mut frame_ready: Box<PsFrameReady> = Box::new(0);
    let mut frame_ready: MaybeUninit<PsFrameReady> = MaybeUninit::uninit();
    match unsafe { Ps2_ReadNextFrame(device_handle, session_index, frame_ready.as_mut_ptr()) } {
        0 => Ok(unsafe { frame_ready.assume_init() }),
        n => Err(n),
    }
}

pub fn get_frame(
    device_handle: PsDeviceHandle,
    session_index: u32,
    frame_type: PsFrameType,
) -> Result<PsFrame, PsReturnStatus> {
    let mut frame: MaybeUninit<PsFrame> = MaybeUninit::uninit();
    match unsafe { Ps2_GetFrame(device_handle, session_index, frame_type, frame.as_mut_ptr()) } {
        0 => Ok(unsafe { frame.assume_init() }),
        n => Err(n),
    }
}

pub fn set_data_mode(
    device_handle: PsDeviceHandle,
    session_index: u32,
    data_mode: PsDataMode,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetDataMode(device_handle, session_index, data_mode) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_data_mode(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<PsDataMode, PsReturnStatus> {
    let mut data_mode: MaybeUninit<PsDataMode> = MaybeUninit::uninit();
    match unsafe { Ps2_GetDataMode(device_handle, session_index, data_mode.as_mut_ptr()) } {
        0 => Ok(unsafe { data_mode.assume_init() }),
        n => Err(n),
    }
}

pub fn get_depth_range(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<PsDepthRange, PsReturnStatus> {
    let mut depth_range: MaybeUninit<PsDepthRange> = MaybeUninit::uninit();
    match unsafe { Ps2_GetDepthRange(device_handle, session_index, depth_range.as_mut_ptr()) } {
        0 => Ok(unsafe { depth_range.assume_init() }),
        n => Err(n),
    }
}

pub fn set_depth_range(
    device_handle: PsDeviceHandle,
    session_index: u32,
    depth_range: PsDepthRange,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetDepthRange(device_handle, session_index, depth_range) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_threshold(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<u16, PsReturnStatus> {
    let mut threshold: u16 = 0;
    match unsafe { Ps2_GetThreshold(device_handle, session_index, &mut threshold) } {
        0 => Ok(threshold),
        n => Err(n),
    }
}

pub fn set_threshold(
    device_handle: PsDeviceHandle,
    session_index: u32,
    threshold: u16,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetThreshold(device_handle, session_index, threshold) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_pulse_count(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<u16, PsReturnStatus> {
    let mut pulse_count: u16 = 0;
    match unsafe { Ps2_GetPulseCount(device_handle, session_index, &mut pulse_count) } {
        0 => Ok(pulse_count),
        n => Err(n),
    }
}

pub fn set_pulse_count(
    device_handle: PsDeviceHandle,
    session_index: u32,
    pulse_count: u16,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetPulseCount(device_handle, session_index, pulse_count) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_gmm_gain(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<u16, PsReturnStatus> {
    let mut gmm_gain: u16 = 0;
    match unsafe { Ps2_GetGMMGain(device_handle, session_index, &mut gmm_gain) } {
        0 => Ok(gmm_gain),
        n => Err(n),
    }
}

pub fn set_gmm_gain(
    device_handle: PsDeviceHandle,
    session_index: u32,
    gmm_gain: u16,
    option: GmmGainEffectiveTime,
) -> Result<(), PsReturnStatus> {
    let gmm_gain: PsGmmGain = PsGmmGain {
        gain: gmm_gain,
        option,
    };
    match unsafe { Ps2_SetGMMGain(device_handle, session_index, gmm_gain) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_property(
    device_handle: PsDeviceHandle,
    session_index: u32,
    property_type: PsPropertyType,
) -> Result<Vec<u8>, PsReturnStatus> {
    let mut size: i32 = 128;
    let mut data_buf: Box<[u8]> = vec![0u8; size as usize].into_boxed_slice();
    let data = data_buf.as_mut_ptr();
    match unsafe {
        Ps2_GetProperty(
            device_handle,
            session_index,
            property_type,
            data as *mut std::ffi::c_void,
            &mut size,
        )
    } {
        0 => Ok(unsafe { Vec::from_raw_parts(data, size as usize, size as usize) }),
        n => Err(n),
    }
}

pub fn set_property(
    device_handle: PsDeviceHandle,
    session_index: u32,
    property_type: PsPropertyType,
    data: PropertyValue,
) -> Result<(), PsReturnStatus> {
    let d = match data {
        PropertyValue::StringValue(s) => (s.as_ptr(), s.as_bytes().len()),
        PropertyValue::Uint8Value(_) | PropertyValue::Int32ValueList(_) => unimplemented!(),
    };
    let data_buf = d.0 as *const std::ffi::c_void;
    let data_size: i32 = d.1 as i32;
    match unsafe {
        Ps2_SetProperty(
            device_handle,
            session_index,
            property_type,
            data_buf,
            data_size,
        )
    } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn get_camera_parameters(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<PsCameraParameters, PsReturnStatus> {
    let mut camera_parameters: MaybeUninit<PsCameraParameters> = MaybeUninit::uninit();
    match unsafe {
        Ps2_GetCameraParameters(device_handle, session_index, camera_parameters.as_mut_ptr())
    } {
        0 => Ok(unsafe { camera_parameters.assume_init() }),
        n => Err(n),
    }
}
