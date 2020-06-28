use std::ffi::CString;
use std::os::raw::c_int;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ZenseError {
    NoDeviceConnected,
    InvalidDeviceIndex,
    DevicePointerIsNull,
    InvalidFrameType,
    FramePointerIsNull,
    NoPropertyValueGet,
    NoPropertyValueSet,
    PropertyPointerIsNull,
    PropertySizeNotEnough,
    InvalidDepthRange,
    ReadNextFrameError,
    InputPointerIsNull,
    CameraNotOpened,
    InvalidCameraType,
    InvalidParams,
    Others,
    FfiError, // Originally added
    Unknown,  // Originally added
}

impl ZenseError {
    pub(crate) fn from_int(n: c_int) -> Self {
        match n {
            -1 => ZenseError::NoDeviceConnected,
            -2 => ZenseError::InvalidDeviceIndex,
            -3 => ZenseError::DevicePointerIsNull,
            -4 => ZenseError::InvalidFrameType,
            -5 => ZenseError::FramePointerIsNull,
            -6 => ZenseError::NoPropertyValueGet,
            -7 => ZenseError::NoPropertyValueSet,
            -8 => ZenseError::PropertyPointerIsNull,
            -9 => ZenseError::PropertySizeNotEnough,
            -10 => ZenseError::InvalidDepthRange,
            -11 => ZenseError::ReadNextFrameError,
            -12 => ZenseError::InputPointerIsNull,
            -13 => ZenseError::CameraNotOpened,
            -14 => ZenseError::InvalidCameraType,
            -15 => ZenseError::InvalidParams,
            -255 => ZenseError::Others,
            _ => ZenseError::Unknown,
        }
    }
}
#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DepthRange {
    Unknown = -1,
    NearRange = 0,
    MidRange,
    FarRange,
    XNearRange,
    XMidRange,
    XFarRange,
    XxNearRange,
    XxMidRange,
    XxFarRange,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DataMode {
    DepthAndRgb30Fps = 0,
    IrAndRGB30Fps = 1,
    DepthAndIr30Fps,
    NoCcd30Fps = 4,
    DepthAndIr15FpsRgb30Fps = 10,
    WdrDepth,
    WdrIr,
    WdrDepthAndIr,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PropertyType {
    SerialNumber = 5,
    FirmwareVersion,
    HardwareVersion,
    DataMode,
    DataModeList,
    DepthRangeList,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PropertyValue {
    StringValue(CString),
    Uint8Value(u8),
    Int32ValueList(Vec<i32>),
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FrameType {
    DepthFrame = 0,
    IrFrame = 1,
    GrayFrame,
    RgbFrame,
    MappedRgbFrame,
    MappedDepthFrame,
    MappedIrFrame,
    ConfidenceFrame = 8,
    WdrDepthFrame,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SensorType {
    DepthSensor = 1,
    RgbSensor = 2,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PixelFormat {
    DepthMm16 = 0,
    Gray16 = 1,
    Gray8,
    Rgb888,
    Bgr888,
}

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

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GmmGainEffectiveTime {
    Temporary = 0u8,
    Permanent = 1u8,
}
