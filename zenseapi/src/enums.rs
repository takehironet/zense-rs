use std::os::raw::c_int;

pub use zenseapi_sys::enums::{GmmGainEffectiveTime, PropertyValue, ZenseError};
use zenseapi_sys::enums::{
    PsDataMode, PsDepthRange, PsFilterType, PsFrameType, PsPixelFormat, PsPropertyType,
    PsResolution, PsSensorType, PsStreamType, PsWdrStyle, PsWdrTotalRange,
};

pub type DepthRange = PsDepthRange;
pub type FrameType = PsFrameType;
pub type PixelFormat = PsPixelFormat;
pub type DataMode = PsDataMode;
pub type PropertyType = PsPropertyType;
pub type SensorType = PsSensorType;
pub type WdrTotalRange = PsWdrTotalRange;
pub type WdrStyle = PsWdrStyle;
pub type FilterType = PsFilterType;
pub type StreamType = PsStreamType;
pub type Resolution = PsResolution;

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
