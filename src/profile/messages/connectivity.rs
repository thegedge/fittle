// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Connectivity {
    bluetooth_enabled: Option<bool>,
    bluetooth_le_enabled: Option<bool>,
    ant_enabled: Option<bool>,
    name: Option<String>,
    live_tracking_enabled: Option<bool>,
    weather_conditions_enabled: Option<bool>,
    weather_alerts_enabled: Option<bool>,
    auto_activity_upload_enabled: Option<bool>,
    course_download_enabled: Option<bool>,
    workout_download_enabled: Option<bool>,
    gps_ephemeris_download_enabled: Option<bool>,
    incident_detection_enabled: Option<bool>,
    grouptrack_enabled: Option<bool>,
}

impl From<Vec<(u8, Field)>> for Connectivity {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.bluetooth_enabled = field.one().map(<bool>::from),
                1 => msg.bluetooth_le_enabled = field.one().map(<bool>::from),
                2 => msg.ant_enabled = field.one().map(<bool>::from),
                3 => msg.name = field.one().map(<String>::from),
                4 => msg.live_tracking_enabled = field.one().map(<bool>::from),
                5 => msg.weather_conditions_enabled = field.one().map(<bool>::from),
                6 => msg.weather_alerts_enabled = field.one().map(<bool>::from),
                7 => msg.auto_activity_upload_enabled = field.one().map(<bool>::from),
                8 => msg.course_download_enabled = field.one().map(<bool>::from),
                9 => msg.workout_download_enabled = field.one().map(<bool>::from),
                10 => msg.gps_ephemeris_download_enabled = field.one().map(<bool>::from),
                11 => msg.incident_detection_enabled = field.one().map(<bool>::from),
                12 => msg.grouptrack_enabled = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

