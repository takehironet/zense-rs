use std::convert::TryInto;

use raw::types::{PsCameraParameters, PsDeviceHandle};
use zenseapi_sys as raw;

use crate::enums::{GmmGainEffectiveTime, PropertyType, PropertyValue};
use crate::{
    ConnectStatus, DataMode, DepthRange, DeviceType, FrameType, PixelFormat, ZenseError,
    ZenseResult,
};

pub type CameraParameters = PsCameraParameters;

macro_rules! handle_check {
    ($self_:ident, $b:expr) => {
        if $self_.device_handle.is_null() {
            Err(ZenseError::FfiError)
        } else if $self_.device_closed {
            Err(ZenseError::CameraNotOpened)
        } else {
            $b
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeviceInfo {
    pub session_count: i64,
    pub device_type: DeviceType,
    pub uri: String,
    pub fw: String,
    pub status: ConnectStatus,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeviceHandle {
    device_handle: PsDeviceHandle,
    device_closed: bool,
}

impl DeviceHandle {
    pub fn new(device_handle: PsDeviceHandle) -> Self {
        DeviceHandle {
            device_handle,
            device_closed: false,
        }
    }

    pub fn close_device(&mut self) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::close_device(self.device_handle) {
                Ok(()) => {
                    self.device_closed = true;
                    Ok(())
                }
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn start_stream(&self, session_index: u32) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::start_stream(self.device_handle, session_index) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn stop_stream(&self, session_index: u32) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::stop_stream(self.device_handle, session_index) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn read_next_frame(&self, session_index: u32) -> ZenseResult<FrameReady> {
        handle_check!(
            self,
            match raw::read_next_frame(self.device_handle, session_index) {
                Ok(frame_ready) => Ok(FrameReady::from(frame_ready)),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_frame(&self, session_index: u32, frame_type: FrameType) -> ZenseResult<Frame> {
        handle_check!(
            self,
            match raw::get_frame(self.device_handle, session_index, frame_type) {
                Ok(frame) => Ok(Frame::from(frame)),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_data_mode(&self, session_index: u32, data_mode: DataMode) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::set_data_mode(self.device_handle, session_index, data_mode) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_data_mode(&self, session_index: u32) -> ZenseResult<DataMode> {
        handle_check!(
            self,
            match raw::get_data_mode(self.device_handle, session_index) {
                Ok(data_mode) => Ok(data_mode),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_depth_range(&self, session_index: u32) -> ZenseResult<DepthRange> {
        handle_check!(
            self,
            match raw::get_depth_range(self.device_handle, session_index) {
                Ok(depth_range) => Ok(depth_range),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_depth_range(&self, session_index: u32, depth_range: DepthRange) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::set_depth_range(self.device_handle, session_index, depth_range) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_threshold(&self, session_index: u32) -> ZenseResult<u16> {
        handle_check!(
            self,
            match raw::get_threshold(self.device_handle, session_index) {
                Ok(threshold) => Ok(threshold),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_threshold(&self, session_index: u32, threshold: u16) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::set_threshold(self.device_handle, session_index, threshold) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_pulse_count(&self, session_index: u32) -> ZenseResult<u16> {
        handle_check!(
            self,
            match raw::get_pulse_count(self.device_handle, session_index) {
                Ok(pulse_count) => Ok(pulse_count),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_pulse_count(&self, session_index: u32, pulse_count: u16) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::set_pulse_count(self.device_handle, session_index, pulse_count) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_gmm_gain(&self, session_index: u32) -> ZenseResult<u16> {
        handle_check!(
            self,
            match raw::get_gmm_gain(self.device_handle, session_index) {
                Ok(gmm_gain) => Ok(gmm_gain),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_gmm_gain(
        &self,
        session_index: u32,
        gmm_gain: u16,
        option: GmmGainEffectiveTime,
    ) -> ZenseResult<()> {
        handle_check!(
            self,
            match raw::set_gmm_gain(self.device_handle, session_index, gmm_gain, option) {
                Ok(()) => Ok(()),
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_property(
        &self,
        session_index: u32,
        property_type: PropertyType,
    ) -> ZenseResult<PropertyValue> {
        handle_check!(
            self,
            match raw::get_property(self.device_handle, session_index, property_type) {
                Ok(property_value) => match property_type {
                    PropertyType::SerialNumber
                    | PropertyType::FirmwareVersion
                    | PropertyType::HardwareVersion => match String::from_utf8(property_value) {
                        Ok(s) => Ok(PropertyValue::StringValue(s)),
                        Err(_) => Err(ZenseError::RuntimeError),
                    },
                    PropertyType::DataMode
                    | PropertyType::DataModeList
                    | PropertyType::DepthRangeList => {
                        todo!("missing DataMode, DataModeList, DepthRangeList implementation")
                    }
                },
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_property(
        &self,
        _session_index: u32,
        _property_type: PropertyType,
        _data: PropertyValue,
    ) -> ZenseResult<()> {
        unimplemented!();
        // handle_check!(
        //     self,
        //     match raw::set_property(self.device_handle, session_index, property_type, data) {
        //         Ok(()) => Ok(()),
        //         Err(n) => Err(ZenseError::from_int(n)),
        //     }
        // )
    }

    pub fn get_camera_parameters(&self, session_index: u32) -> ZenseResult<CameraParameters> {
        handle_check!(
            self,
            match raw::get_camera_parameters(self.device_handle, session_index) {
                Ok(camera_parameters) => {
                    let error = 0.001f64;
                    if (camera_parameters.cx - f64::default()).abs() < error
                        && (camera_parameters.cy - f64::default()).abs() < error
                        && (camera_parameters.fx - f64::default()).abs() < error
                        && (camera_parameters.fy - f64::default()).abs() < error
                        && (camera_parameters.k1 - f64::default()).abs() < error
                        && (camera_parameters.k2 - f64::default()).abs() < error
                        && (camera_parameters.k3 - f64::default()).abs() < error
                        && (camera_parameters.k4 - f64::default()).abs() < error
                        && (camera_parameters.k5 - f64::default()).abs() < error
                        && (camera_parameters.k6 - f64::default()).abs() < error
                        && (camera_parameters.p1 - f64::default()).abs() < error
                        && (camera_parameters.p2 - f64::default()).abs() < error
                    {
                        Err(ZenseError::FfiError)
                    } else {
                        Ok(camera_parameters)
                    }
                }
                Err(n) => Err(ZenseError::from_int(n)),
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FrameReady {
    pub depth: bool,
    pub ir: bool,
    pub rgb: bool,
    pub mapped_rgb: bool,
    pub mapped_depth: bool,
    pub mapped_ir: bool,
    pub confidence: bool,
    pub wdr_depth: bool,
    pub unknown: bool, // defined as reserved bits in VzenseSDK header file
}

impl FrameReady {
    pub(crate) fn from(frame_ready: raw::types::PsFrameReady) -> Self {
        FrameReady {
            depth: (frame_ready & 0x01_u32) != 0,
            ir: (frame_ready & 0x02_u32) != 0,
            rgb: (frame_ready & 0x04_u32) != 0,
            mapped_rgb: (frame_ready & 0x08_u32) != 0,
            mapped_depth: (frame_ready & 0x10_u32) != 0,
            mapped_ir: (frame_ready & 0x20_u32) != 0,
            confidence: (frame_ready & 0x40_u32) != 0,
            wdr_depth: (frame_ready & 0x80_u32) != 0,
            unknown: (frame_ready & (!0xff_u32)) != 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub frame_index: u32,
    pub frame_type: FrameType,
    pub pixel_format: PixelFormat,
    pub imu_frame_no: u8,
    pub frame_data: Vec<u8>,
    pub exposure_time: f32,
    pub depth_range: DepthRange,
    pub width: u16,
    pub height: u16,
}

impl Frame {
    pub(crate) fn from(ps_frame: raw::types::PsFrame) -> Self {
        let data_length: usize = ps_frame.data_len.try_into().unwrap();
        let frame_data: Vec<u8> =
            unsafe { std::slice::from_raw_parts(ps_frame.frame_data, data_length) }.to_vec();
        Frame {
            frame_index: ps_frame.frame_index,
            frame_type: ps_frame.frame_type,
            pixel_format: ps_frame.pixel_format,
            imu_frame_no: ps_frame.imu_frame_no,
            frame_data,
            exposure_time: ps_frame.exposure_time,
            depth_range: ps_frame.depth_range,
            width: ps_frame.width,
            height: ps_frame.height,
        }
    }
}
