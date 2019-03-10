// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct CameraEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    camera_event_type: Option<enums::CameraEventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    camera_file_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    camera_orientation: Option<enums::CameraOrientationType>,

}

impl CameraEvent {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = content.one().map(<u16>::from),
                1 => msg.camera_event_type = content.one().map(<enums::CameraEventType>::from),
                2 => msg.camera_file_uuid = content.one().map(<String>::from),
                3 => msg.camera_orientation = content.one().map(<enums::CameraOrientationType>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

