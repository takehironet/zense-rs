use std::ffi::c_void;
use std::os::raw::c_char;

use crate::enums::{PsPixelFormat, PsResolution, PsSensorType, PsWdrStyle};
use crate::types::{
    PsCameraParameters, PsDeviceHandle, PsFrameReady, PsReturnStatus, PsWdrOutputMode,
};
use crate::{
    enums::PsPropertyType, PsDataMode, PsDepthRange, PsDeviceInfo, PsFrame, PsFrameType, PsGmmGain,
};

#[link(name = "vzense_api")]
extern "C" {
    pub fn Ps2_Initialize() -> PsReturnStatus;
    pub fn Ps2_Shutdown() -> PsReturnStatus;
    pub fn Ps2_GetDeviceCount(device_count: *mut u32) -> PsReturnStatus;
    pub fn Ps2_GetDeviceListInfo(
        devices_list: *mut PsDeviceInfo,
        device_count: u32,
    ) -> PsReturnStatus;
    pub fn Ps2_GetDeviceInfo(device_info: *mut PsDeviceInfo, device_index: u32) -> PsReturnStatus;
    pub fn Ps2_OpenDevice(
        uri: *const c_char,
        device_handler: *mut PsDeviceHandle,
    ) -> PsReturnStatus;
    pub fn Ps2_CloseDevice(device_handle: PsDeviceHandle) -> PsReturnStatus;
    pub fn Ps2_StartStream(device_handle: PsDeviceHandle, session_index: u32) -> PsReturnStatus;
    pub fn Ps2_StopStream(device_handle: PsDeviceHandle, session_index: u32) -> PsReturnStatus;
    pub fn Ps2_ReadNextFrame(
        device_handle: PsDeviceHandle,
        session_index: u32,
        frame_ready: *mut PsFrameReady,
    ) -> PsReturnStatus;
    pub fn Ps2_GetFrame(
        device_handle: PsDeviceHandle,
        session_index: u32,
        frame_type: PsFrameType,
        frame: *mut PsFrame,
    ) -> PsReturnStatus;

    pub fn Ps2_SetDataMode(
        device_handle: PsDeviceHandle,
        session_index: u32,
        data_mode: PsDataMode,
    ) -> PsReturnStatus;
    pub fn Ps2_GetDataMode(
        device_handle: PsDeviceHandle,
        session_index: u32,
        data_mode: *mut PsDataMode,
    ) -> PsReturnStatus;

    pub fn Ps2_GetDepthRange(
        device_handle: PsDeviceHandle,
        session_index: u32,
        depth_range: *mut PsDepthRange,
    ) -> PsReturnStatus;
    pub fn Ps2_SetDepthRange(
        device_handle: PsDeviceHandle,
        session_index: u32,
        depth_range: PsDepthRange,
    ) -> PsReturnStatus;

    pub fn Ps2_GetThreshold(
        device_handle: PsDeviceHandle,
        session_index: u32,
        threshold: *mut u16,
    ) -> PsReturnStatus;
    pub fn Ps2_SetThreshold(
        device_handle: PsDeviceHandle,
        session_index: u32,
        threshold: u16,
    ) -> PsReturnStatus;

    pub fn Ps2_GetPulseCount(
        device_handle: PsDeviceHandle,
        session_index: u32,
        pulse_count: *mut u16,
    ) -> PsReturnStatus;
    pub fn Ps2_SetPulseCount(
        device_handle: PsDeviceHandle,
        session_index: u32,
        pulse_count: u16,
    ) -> PsReturnStatus;

    pub fn Ps2_GetGMMGain(
        device_handle: PsDeviceHandle,
        session_index: u32,
        gain: *mut u16,
    ) -> PsReturnStatus;
    pub fn Ps2_SetGMMGain(
        device_handle: PsDeviceHandle,
        session_index: u32,
        gmm_gain: PsGmmGain,
    ) -> PsReturnStatus;

    pub fn Ps2_GetProperty(
        device_handle: PsDeviceHandle,
        session_index: u32,
        property_type: PsPropertyType,
        data: *mut i8,
        data_size: *mut i32, // in/out
    ) -> PsReturnStatus;
    pub fn Ps2_SetProperty(
        device_handle: PsDeviceHandle,
        session_index: u32,
        property_type: PsPropertyType,
        data: *const c_void,
        data_size: i32,
    ) -> PsReturnStatus;

    pub fn Ps2_GetCameraParameters(
        device_handle: PsDeviceHandle,
        session_index: u32,
        sensor_type: PsSensorType,
        camera_parameters: *mut PsCameraParameters,
    ) -> PsReturnStatus;

    //

    pub fn Ps2_SetWDROutputMode(
        device_handle: PsDeviceHandle,
        session_index: u32,
        wdr_mode: *const PsWdrOutputMode,
    ) -> PsReturnStatus;

    pub fn Ps2_SetWDRStyle(
        device_handle: PsDeviceHandle,
        session_index: u32,
        wdr_style: PsWdrStyle,
    ) -> PsReturnStatus;

    pub fn Ps2_SetRgbFrameEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetDepthDistortionCorrectionEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetIrDistortionCorrectionEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetRGBDistortionCorrectionEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetComputeRealDepthCorrectionEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetSpatialFilterEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetTimeFilterEnabled(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetMapperEnabledRGBToDepth(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetMapperEnabledDepthToRGB(
        device_handle: PsDeviceHandle,
        session_index: u32,
        enabled: bool,
    ) -> PsReturnStatus;

    pub fn Ps2_SetRGBResolution(
        device_handle: PsDeviceHandle,
        session_index: u32,
        resolution: PsResolution,
    ) -> PsReturnStatus;

    pub fn Ps2_SetColorPixelFormat(
        device_handle: PsDeviceHandle,
        session_index: u32,
        pixel_format: PsPixelFormat,
    ) -> PsReturnStatus;
}
