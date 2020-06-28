use std::ffi::c_void;
use std::os::raw::{c_char, c_float, c_int};

use crate::enums::{GmmGainEffectiveTime, PropertyType};
use crate::{DataMode, DepthRange, FrameType, PixelFormat};

type PsReturnStatus = c_int;

#[link(name = "vzense_api")]
extern "C" {
    pub(crate) fn Ps2_Initialize() -> PsReturnStatus;
    pub(crate) fn Ps2_Shutdown() -> PsReturnStatus;
    pub(crate) fn Ps2_GetDeviceCount(device_count: *const u32) -> PsReturnStatus;
    pub(crate) fn Ps2_GetDeviceListInfo(
        devices_list: *mut PsDeviceInfo,
        device_count: u32,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_GetDeviceInfo(
        device_info: *mut PsDeviceInfo,
        device_index: u32,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_OpenDevice(
        uri: *const c_char,
        device_handler: *mut PsDeviceHandle,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_CloseDevice(device_handle: PsDeviceHandle) -> PsReturnStatus;
    pub(crate) fn Ps2_StartStream(
        device_handle: PsDeviceHandle,
        session_index: u32,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_StopStream(
        device_handle: PsDeviceHandle,
        session_index: u32,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_ReadNextFrame(
        device_handle: PsDeviceHandle,
        session_index: u32,
        frame_ready: Option<&mut PsFrameReady>,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_GetFrame(
        device_handle: PsDeviceHandle,
        session_index: u32,
        frame_type: FrameType,
        frame: Option<&mut PsFrame>,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_SetDataMode(
        device_handle: PsDeviceHandle,
        session_index: u32,
        data_mode: DataMode,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_GetDataMode(
        device_handle: PsDeviceHandle,
        session_index: u32,
        data_mode: Option<&mut DataMode>,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_GetDepthRange(
        device_handle: PsDeviceHandle,
        session_index: u32,
        depth_range: Option<&mut DepthRange>,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_SetDepthRange(
        device_handle: PsDeviceHandle,
        session_index: u32,
        depth_range: DepthRange,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_GetThreshold(
        device_handle: PsDeviceHandle,
        session_index: u32,
        threshold: Option<&mut u16>,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_SetThreshold(
        device_handle: PsDeviceHandle,
        session_index: u32,
        threshold: u16,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_GetPulseCount(
        device_handle: PsDeviceHandle,
        session_index: u32,
        pulse_count: Option<&mut u16>,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_SetPulseCount(
        device_handle: PsDeviceHandle,
        session_index: u32,
        pulse_count: u16,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_GetGMMGain(
        device_handle: PsDeviceHandle,
        session_index: u32,
        gain: Option<&mut u16>,
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_SetGMMGain(
        device_handle: PsDeviceHandle,
        session_index: u32,
        gmm_gain: PsGmmGain,
    ) -> PsReturnStatus;

    pub(crate) fn Ps2_GetProperty(
        device_handle: PsDeviceHandle,
        session_index: u32,
        property_type: PropertyType,
        data: *mut c_void,
        data_size: *mut i32, // in/out
    ) -> PsReturnStatus;
    pub(crate) fn Ps2_SetProperty(
        device_handle: PsDeviceHandle,
        session_index: u32,
        property_type: PropertyType,
        data: *const c_void,
        data_size: i32,
    ) -> PsReturnStatus;
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct PsDeviceInfo {
    pub session_count: c_int,
    pub device_type: c_int,
    pub uri: [c_char; 256usize],
    pub fw: [c_char; 50usize],
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct Device {
    _private: [u8; 0],
}

pub(crate) type PsDeviceHandle = *mut Device;

pub(crate) type PsFrameReady = u32;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct PsFrame {
    pub(crate) frame_index: u32,
    pub(crate) frame_type: FrameType,
    pub(crate) pixel_format: PixelFormat,
    pub(crate) imu_frame_no: u8,
    pub(crate) frame_data: *mut u8,
    pub(crate) data_len: u32,
    pub(crate) exposure_time: c_float,
    pub(crate) depth_range: DepthRange,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct PsGmmGain {
    pub(crate) gain: u16,
    pub(crate) option: GmmGainEffectiveTime,
}
