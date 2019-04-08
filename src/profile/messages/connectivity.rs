// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

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
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.bluetooth_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            1 => {
                self.bluetooth_le_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            2 => {
                self.ant_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            3 => {
                self.name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            4 => {
                self.live_tracking_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            5 => {
                self.weather_conditions_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            6 => {
                self.weather_alerts_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            7 => {
                self.auto_activity_upload_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            8 => {
                self.course_download_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            9 => {
                self.workout_download_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            10 => {
                self.gps_ephemeris_download_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            11 => {
                self.incident_detection_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            12 => {
                self.grouptrack_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
