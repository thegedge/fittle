// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Connectivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    ant_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_activity_upload_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bluetooth_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bluetooth_le_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    course_download_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_ephemeris_download_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grouptrack_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    incident_detection_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    live_tracking_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weather_alerts_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weather_conditions_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    workout_download_enabled: Option<bool>,
}

impl Connectivity {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.bluetooth_enabled = content.one().map(<bool>::from),
                1 => msg.bluetooth_le_enabled = content.one().map(<bool>::from),
                2 => msg.ant_enabled = content.one().map(<bool>::from),
                3 => msg.name = content.one().map(<String>::from),
                4 => msg.live_tracking_enabled = content.one().map(<bool>::from),
                5 => msg.weather_conditions_enabled = content.one().map(<bool>::from),
                6 => msg.weather_alerts_enabled = content.one().map(<bool>::from),
                7 => msg.auto_activity_upload_enabled = content.one().map(<bool>::from),
                8 => msg.course_download_enabled = content.one().map(<bool>::from),
                9 => msg.workout_download_enabled = content.one().map(<bool>::from),
                10 => msg.gps_ephemeris_download_enabled = content.one().map(<bool>::from),
                11 => msg.incident_detection_enabled = content.one().map(<bool>::from),
                12 => msg.grouptrack_enabled = content.one().map(<bool>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
