use std::os::raw::c_int;

pub use zenseapi_sys::enums::{GmmGainEffectiveTime, PropertyValue, SensorType, ZenseError};
use zenseapi_sys::enums::{PsDataMode, PsDepthRange, PsFrameType, PsPixelFormat, PsPropertyType};

pub type DepthRange = PsDepthRange;
pub type FrameType = PsFrameType;
pub type PixelFormat = PsPixelFormat;
pub type DataMode = PsDataMode;
pub type PropertyType = PsPropertyType;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FilterType {
    ComputeRealDepthFilter,
    SmoothingFilter,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WdrTotalRange {
    WdrTotalRangeTwo = 2,
    WdrTotalRangeThree = 3,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WdrStyle {
    Fusion,
    Alternation,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum StreamType {
    Depth,
    Ir,
    Rgb,
    Audio,
    Imu,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Resolution {
    Res1920x1080,
    Res1280x720,
    Res640x480,
    Res640x360,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum LinkType {
    Unknown,
    Usb,
    Socket,
    Mipi,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ConnectStatus {
    ConnectUnknown,
    Unconnected,
    Connected,
    Opened,
}

impl ConnectStatus {
    pub(crate) fn from_int(n: c_int) -> Self {
        match n {
            1 => ConnectStatus::Unconnected,
            2 => ConnectStatus::Connected,
            3 => ConnectStatus::Opened,
            _ => ConnectStatus::ConnectUnknown,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DeviceType {
    Unknown = -1,
    None = 0,
    DcamUpdate = 1,
    Dcam305 = 305,
    Dcam500 = 500,
    Dcam700 = 700,
    Dcam710 = 710,
    Dcam800 = 800,
    DcamMipi = 801,
    Dcam800Lite = 802,
    Max,
}

impl DeviceType {
    pub(crate) fn from_int(n: c_int) -> Self {
        match n {
            0 => DeviceType::None,
            1 => DeviceType::DcamUpdate,
            305 => DeviceType::Dcam305,
            500 => DeviceType::Dcam500,
            700 => DeviceType::Dcam700,
            710 => DeviceType::Dcam710,
            800 => DeviceType::Dcam800,
            801 => DeviceType::DcamMipi,
            802 => DeviceType::Dcam800Lite,
            803 => DeviceType::Max,
            _ => DeviceType::Unknown,
        }
    }
}
