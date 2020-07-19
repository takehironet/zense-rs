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
    FfiError,
    // Originally added
    RuntimeError,
    // Originally added
    Unknown, // Originally added
}

impl ZenseError {
    pub fn from_int(n: c_int) -> Self {
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
            254 => ZenseError::RuntimeError,
            255 => ZenseError::FfiError,
            _ => ZenseError::Unknown,
        }
    }
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsDepthRange {
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
pub enum PsDataMode {
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
pub enum PsPropertyType {
    SerialNumber = 5,
    FirmwareVersion,
    HardwareVersion,
    DataMode,
    DataModeList,
    DepthRangeList,
}

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PropertyValue {
    StringValue(CString),
    Uint8Value(u8),
    Int32ValueList(Vec<i32>),
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsFrameType {
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

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsSensorType {
    DepthSensor = 1,
    RgbSensor = 2,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsPixelFormat {
    DepthMm16 = 0,
    Gray16 = 1,
    Gray8,
    Rgb888,
    Bgr888,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsFilterType {
    ComputeRealDepthFilter,
    SmoothingFilter,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsWdrTotalRange {
    WdrTotalRangeTwo = 2,
    WdrTotalRangeThree = 3,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsWdrStyle {
    Fusion,
    Alternation,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsStreamType {
    Depth,
    Ir,
    Rgb,
    Audio,
    Imu,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsResolution {
    Res1920x1080,
    Res1280x720,
    Res640x480,
    Res640x360,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsLinkType {
    Unknown,
    Usb,
    Socket,
    Mipi,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsConnectStatus {
    ConnectUnknown,
    Unconnected,
    Connected,
    Opened,
}

impl PsConnectStatus {
    pub fn from_int(n: c_int) -> Self {
        match n {
            1 => PsConnectStatus::Unconnected,
            2 => PsConnectStatus::Connected,
            3 => PsConnectStatus::Opened,
            _ => PsConnectStatus::ConnectUnknown,
        }
    }
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PsDeviceType {
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

impl PsDeviceType {
    pub fn from_int(n: c_int) -> Self {
        match n {
            0 => PsDeviceType::None,
            1 => PsDeviceType::DcamUpdate,
            305 => PsDeviceType::Dcam305,
            500 => PsDeviceType::Dcam500,
            700 => PsDeviceType::Dcam700,
            710 => PsDeviceType::Dcam710,
            800 => PsDeviceType::Dcam800,
            801 => PsDeviceType::DcamMipi,
            802 => PsDeviceType::Dcam800Lite,
            803 => PsDeviceType::Max,
            _ => PsDeviceType::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GmmGainEffectiveTime {
    Temporary = 0u8,
    Permanent = 1u8,
}
