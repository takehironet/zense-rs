use std::ffi::CString;
use std::mem::MaybeUninit;
use std::num::NonZeroU8;

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
    let mut frame_data: [u8; 0] = [0; 0];
    let mut frame = PsFrame {
        frame_index: 0,
        frame_type,
        pixel_format: PsPixelFormat::Bgr888,
        imu_frame_no: 0,
        frame_data: frame_data.as_mut_ptr(),
        data_len: 0,
        exposure_time: 0.0,
        depth_range: PsDepthRange::Unknown,
        width: 0,
        height: 0,
    };
    match unsafe { Ps2_GetFrame(device_handle, session_index, frame_type, &mut frame) } {
        0 => Ok(frame),
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
    let mut data_mode: PsDataMode = PsDataMode::DepthAndRgb30Fps;
    match unsafe { Ps2_GetDataMode(device_handle, session_index, &mut data_mode) } {
        0 => Ok(data_mode),
        n => Err(n),
    }
}

pub fn get_depth_range(
    device_handle: PsDeviceHandle,
    session_index: u32,
) -> Result<PsDepthRange, PsReturnStatus> {
    let mut depth_range: PsDepthRange = PsDepthRange::Unknown;
    match unsafe { Ps2_GetDepthRange(device_handle, session_index, &mut depth_range) } {
        0 => Ok(depth_range),
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
) -> Result<PropertyValue, PsReturnStatus> {
    let mut size: i32 = 128;
    let mut data_buf = Vec::<i8>::with_capacity(size as usize);
    let data = data_buf.as_mut_ptr();
    match unsafe { Ps2_GetProperty(device_handle, session_index, property_type, data, &mut size) } {
        0 => match property_type {
            PsPropertyType::SerialNumber
            | PsPropertyType::FirmwareVersion
            | PsPropertyType::HardwareVersion => {
                if data.is_null() {
                    Err(255)
                } else {
                    let c_char_slice = unsafe { std::slice::from_raw_parts(data, size as usize) };
                    let vec_nz_u8 = c_char_slice
                        .iter()
                        .map(|&x| x as u8)
                        .take_while(|&x| x != 0)
                        .map(|x| NonZeroU8::new(x).unwrap())
                        .collect::<Vec<NonZeroU8>>();
                    let cstring = CString::from(vec_nz_u8);
                    Ok(PropertyValue::StringValue(cstring))
                }
            }
            PsPropertyType::DataMode
            | PsPropertyType::DataModeList
            | PsPropertyType::DepthRangeList => unimplemented!(),
        },
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
    sensor_type: PsSensorType,
) -> Result<PsCameraParameters, PsReturnStatus> {
    let mut camera_parameters: MaybeUninit<PsCameraParameters> = MaybeUninit::uninit();
    match unsafe {
        Ps2_GetCameraParameters(
            device_handle,
            session_index,
            sensor_type,
            camera_parameters.as_mut_ptr(),
        )
    } {
        0 => Ok(unsafe { camera_parameters.assume_init() }),
        n => Err(n),
    }
}

pub fn set_wdr_output_mode(
    device_handle: PsDeviceHandle,
    session_index: u32,
    wdr_mode: PsWdrOutputMode,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetWDROutputMode(device_handle, session_index, &wdr_mode) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_wdr_style(
    device_handle: PsDeviceHandle,
    session_index: u32,
    wdr_style: PsWdrStyle,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetWDRStyle(device_handle, session_index, wdr_style) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_rgb_frame_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetRgbFrameEnabled(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_depth_distortion_correction_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetDepthDistortionCorrectionEnabled(device_handle, session_index, enabled) }
    {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_ir_distortion_correction_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetIrDistortionCorrectionEnabled(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_rgb_distortion_correction_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetRGBDistortionCorrectionEnabled(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_compute_real_depth_correction_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetComputeRealDepthCorrectionEnabled(device_handle, session_index, enabled) }
    {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_spatial_filter_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetSpatialFilterEnabled(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_time_filter_enabled(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetTimeFilterEnabled(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_mapper_enabled_rgb_to_depth(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetMapperEnabledRGBToDepth(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_mapper_enabled_depth_to_rgb(
    device_handle: PsDeviceHandle,
    session_index: u32,
    enabled: bool,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetMapperEnabledDepthToRGB(device_handle, session_index, enabled) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_rgb_resolution(
    device_handle: PsDeviceHandle,
    session_index: u32,
    resolution: PsResolution,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetRGBResolution(device_handle, session_index, resolution) } {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn set_color_pixel_format(
    device_handle: PsDeviceHandle,
    session_index: u32,
    pixel_format: PsPixelFormat,
) -> Result<(), PsReturnStatus> {
    match unsafe { Ps2_SetColorPixelFormat(device_handle, session_index, pixel_format) } {
        0 => Ok(()),
        n => Err(n),
    }
}
