use std::os::raw::{c_char, c_float, c_int};

use crate::enums::{GmmGainEffectiveTime, PsDepthRange, PsFrameType, PsPixelFormat};

pub type PsReturnStatus = c_int;
pub type PsDeviceHandle = *mut Device;
pub type PsFrameReady = u32;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PsDeviceInfo {
    pub session_count: c_int,
    pub device_type: c_int,
    pub uri: [c_char; 256usize],
    pub fw: [c_char; 50usize],
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PsFrame {
    pub frame_index: u32,
    pub frame_type: PsFrameType,
    pub pixel_format: PsPixelFormat,
    pub imu_frame_no: u8,
    pub frame_data: *mut u8,
    pub data_len: u32,
    pub exposure_time: c_float,
    pub depth_range: PsDepthRange,
    pub width: u16,
    pub height: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PsGmmGain {
    pub gain: u16,
    pub option: GmmGainEffectiveTime,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PsCameraParameters {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: f64,
    pub k2: f64,
    pub p1: f64,
    pub p2: f64,
    pub k3: f64,
    pub k4: f64,
    pub k5: f64,
    pub k6: f64,
}
