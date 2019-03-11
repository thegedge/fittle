use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityCapabilities {
    ActivityUpload,
    Ant,
    AudioPrompts,
    Bluetooth,
    BluetoothLe,
    ConnectIqAppDownload,
    ConnectIqAppManagment,
    ConnectIqDataFieldDownload,
    ConnectIqWatchAppDownload,
    ConnectIqWatchFaceDownload,
    ConnectIqWidgetDownload,
    ContinueSyncAfterSoftwareUpdate,
    CourseDownload,
    DeviceInitiatesSync,
    ExplicitArchive,
    FindMyWatch,
    GolfCourseDownload,
    GpsEphemerisDownload,
    IncidentDetection,
    InstantInput,
    LiveTrack,
    LiveTrackAutoStart,
    LiveTrackMessaging,
    RemoteManualSync,
    SetupIncomplete,
    SwingSensor,
    SwingSensorRemote,
    TrueUp,
    WeatherAlerts,
    WeatherConditions,
    WifiVerification,
    WorkoutDownload,
    UnknownValue(u64),
}

impl From<FieldContent> for ConnectivityCapabilities {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32z(enum_value) = field {
            match enum_value {
                1 => ConnectivityCapabilities::Bluetooth,
                2 => ConnectivityCapabilities::BluetoothLe,
                4 => ConnectivityCapabilities::Ant,
                8 => ConnectivityCapabilities::ActivityUpload,
                16 => ConnectivityCapabilities::CourseDownload,
                32 => ConnectivityCapabilities::WorkoutDownload,
                64 => ConnectivityCapabilities::LiveTrack,
                128 => ConnectivityCapabilities::WeatherConditions,
                256 => ConnectivityCapabilities::WeatherAlerts,
                512 => ConnectivityCapabilities::GpsEphemerisDownload,
                1024 => ConnectivityCapabilities::ExplicitArchive,
                2048 => ConnectivityCapabilities::SetupIncomplete,
                4096 => ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate,
                8192 => ConnectivityCapabilities::ConnectIqAppDownload,
                16384 => ConnectivityCapabilities::GolfCourseDownload,
                32768 => ConnectivityCapabilities::DeviceInitiatesSync,
                65536 => ConnectivityCapabilities::ConnectIqWatchAppDownload,
                131072 => ConnectivityCapabilities::ConnectIqWidgetDownload,
                262144 => ConnectivityCapabilities::ConnectIqWatchFaceDownload,
                524288 => ConnectivityCapabilities::ConnectIqDataFieldDownload,
                1048576 => ConnectivityCapabilities::ConnectIqAppManagment,
                2097152 => ConnectivityCapabilities::SwingSensor,
                4194304 => ConnectivityCapabilities::SwingSensorRemote,
                8388608 => ConnectivityCapabilities::IncidentDetection,
                16777216 => ConnectivityCapabilities::AudioPrompts,
                33554432 => ConnectivityCapabilities::WifiVerification,
                67108864 => ConnectivityCapabilities::TrueUp,
                134217728 => ConnectivityCapabilities::FindMyWatch,
                268435456 => ConnectivityCapabilities::RemoteManualSync,
                536870912 => ConnectivityCapabilities::LiveTrackAutoStart,
                1073741824 => ConnectivityCapabilities::LiveTrackMessaging,
                2147483648 => ConnectivityCapabilities::InstantInput,
                n => ConnectivityCapabilities::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ConnectivityCapabilities to {:?}", field);
        }
    }
}
