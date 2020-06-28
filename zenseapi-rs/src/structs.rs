use std::convert::TryInto;

use crate::enums::{GmmGainEffectiveTime, PropertyType, PropertyValue};
use crate::ffi::{
    Ps2_CloseDevice, Ps2_GetDataMode, Ps2_GetDepthRange, Ps2_GetFrame, Ps2_GetGMMGain,
    Ps2_GetProperty, Ps2_GetPulseCount, Ps2_GetThreshold, Ps2_ReadNextFrame, Ps2_SetDataMode,
    Ps2_SetDepthRange, Ps2_SetGMMGain, Ps2_SetProperty, Ps2_SetPulseCount, Ps2_SetThreshold,
    Ps2_StartStream, Ps2_StopStream, PsDeviceHandle, PsFrame, PsFrameReady, PsGmmGain,
};
use crate::{
    ConnectStatus, DataMode, DepthRange, DeviceType, FrameType, PixelFormat, ZenseError,
    ZenseResult,
};
use std::borrow::BorrowMut;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::slice_from_raw_parts;

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
    pub(crate) fn new(device_handle: PsDeviceHandle) -> Self {
        DeviceHandle {
            device_handle,
            device_closed: false,
        }
    }

    pub fn close_device(&mut self) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_CloseDevice(self.device_handle) } {
                0 => {
                    self.device_closed = true;
                    Ok(())
                }
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn start_stream(&self, session_index: u32) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_StartStream(self.device_handle, session_index) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn stop_stream(&self, session_index: u32) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_StopStream(self.device_handle, session_index) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn read_next_frame(&self, session_index: u32) -> ZenseResult<FrameReady> {
        let mut frame_ready: Box<PsFrameReady> = Box::new(0);
        handle_check!(
            self,
            match unsafe {
                Ps2_ReadNextFrame(
                    self.device_handle,
                    session_index,
                    Some(frame_ready.as_mut()),
                )
            } {
                0 => {
                    Ok(FrameReady::from(*frame_ready))
                }
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_frame(&self, session_index: u32, frame_type: FrameType) -> ZenseResult<Frame> {
        let mut frame: PsFrame = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        handle_check!(
            self,
            match unsafe {
                Ps2_GetFrame(
                    self.device_handle,
                    session_index,
                    frame_type,
                    Some(&mut frame),
                )
            } {
                0 => {
                    Ok(Frame::from(frame))
                }
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_data_mode(&self, session_index: u32, data_mode: DataMode) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_SetDataMode(self.device_handle, session_index, data_mode) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_data_mode(&self, session_index: u32) -> ZenseResult<DataMode> {
        let mut data_mode: DataMode = DataMode::DepthAndRgb30Fps;
        handle_check!(
            self,
            match unsafe {
                Ps2_GetDataMode(self.device_handle, session_index, Some(&mut data_mode))
            } {
                0 => Ok(data_mode),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_depth_range(&self, session_index: u32) -> ZenseResult<DepthRange> {
        let mut depth_range: DepthRange = DepthRange::Unknown;
        handle_check!(
            self,
            match unsafe {
                Ps2_GetDepthRange(self.device_handle, session_index, Some(&mut depth_range))
            } {
                0 => Ok(depth_range),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_depth_range(&self, session_index: u32, depth_range: DepthRange) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_SetDepthRange(self.device_handle, session_index, depth_range) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_threshold(&self, session_index: u32) -> ZenseResult<u16> {
        let mut threshold: u16 = 0;
        handle_check!(
            self,
            match unsafe {
                Ps2_GetThreshold(self.device_handle, session_index, Some(&mut threshold))
            } {
                0 => Ok(threshold),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_threshold(&self, session_index: u32, threshold: u16) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_SetThreshold(self.device_handle, session_index, threshold) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_pulse_count(&self, session_index: u32) -> ZenseResult<u16> {
        let mut pulse_count: u16 = 0;
        handle_check!(
            self,
            match unsafe {
                Ps2_GetPulseCount(self.device_handle, session_index, Some(&mut pulse_count))
            } {
                0 => Ok(pulse_count),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_pulse_count(&self, session_index: u32, pulse_count: u16) -> ZenseResult<()> {
        handle_check!(
            self,
            match unsafe { Ps2_SetPulseCount(self.device_handle, session_index, pulse_count) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_gmm_gain(&self, session_index: u32) -> ZenseResult<u16> {
        let mut gmm_gain: u16 = 0;
        handle_check!(
            self,
            match unsafe { Ps2_GetGMMGain(self.device_handle, session_index, Some(&mut gmm_gain)) }
            {
                0 => Ok(gmm_gain),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn set_gmm_gain(
        &self,
        session_index: u32,
        gmm_gain: u16,
        option: GmmGainEffectiveTime,
    ) -> ZenseResult<()> {
        let gmm_gain: PsGmmGain = PsGmmGain {
            gain: gmm_gain,
            option,
        };
        handle_check!(
            self,
            match unsafe { Ps2_SetGMMGain(self.device_handle, session_index, gmm_gain) } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
            }
        )
    }

    pub fn get_property(
        &self,
        session_index: u32,
        property_type: PropertyType,
    ) -> ZenseResult<PropertyValue> {
        let mut size: i32 = 128;
        let mut data_buf: Box<[u8]> = vec![0u8; size as usize].into_boxed_slice();
        let data = data_buf.as_mut_ptr();
        let res = match unsafe {
            Ps2_GetProperty(
                self.device_handle,
                session_index,
                property_type,
                data as *mut std::ffi::c_void,
                &mut size,
            )
        } {
            0 => match property_type {
                PropertyType::SerialNumber
                | PropertyType::FirmwareVersion
                | PropertyType::HardwareVersion => {
                    let s = unsafe { CStr::from_ptr(data as *mut c_char) };
                    Ok(PropertyValue::StringValue( CString::from(s)))
                }
                PropertyType::DataMode |
                PropertyType::DataModeList |
                PropertyType::DepthRangeList => unimplemented!()
                // PropertyType::DataMode => PropertyValue::Uint8Value(0u8),
                // PropertyType::DataModeList => PropertyValue::Int32ValueList(vec![]),
                // PropertyType::DepthRangeList => PropertyValue::Int32ValueList(vec![]),
            },
            n if n > 0 => Err(ZenseError::FfiError),
            n => Err(ZenseError::from_int(n)),
        };
        handle_check!(self, res)
    }

    pub fn set_property(
        &self,
        session_index: u32,
        property_type: PropertyType,
        data: PropertyValue,
    ) -> ZenseResult<()> {
        let d = match data {
            PropertyValue::StringValue(s) => (s.as_ptr(), s.as_bytes().len()),
            PropertyValue::Uint8Value(_) | PropertyValue::Int32ValueList(_) => unimplemented!(),
        };
        let data_buf = d.0 as *const std::ffi::c_void;
        let data_size: i32 = d.1 as i32;
        handle_check!(
            self,
            match unsafe {
                Ps2_SetProperty(
                    self.device_handle,
                    session_index,
                    property_type,
                    data_buf,
                    data_size,
                )
            } {
                0 => Ok(()),
                n if n > 0 => Err(ZenseError::FfiError),
                n => Err(ZenseError::from_int(n)),
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
    pub(crate) fn from(frame_ready: PsFrameReady) -> Self {
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
    pub(crate) fn from(ps_frame: PsFrame) -> Self {
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
