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
pub struct CameraEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    camera_event_type: Option<crate::profile::enums::CameraEventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    camera_file_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    camera_orientation: Option<crate::profile::enums::CameraOrientationType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,
}

impl CameraEvent {
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
                self.timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u16>)(value)
                })
            },

            1 => {
                self.camera_event_type =field.one().map(|v| {
                    let value = crate::profile::enums::CameraEventType::from(v);
                    value
                })
            },

            2 => {
                self.camera_file_uuid =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            3 => {
                self.camera_orientation =field.one().map(|v| {
                    let value = crate::profile::enums::CameraOrientationType::from(v);
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
