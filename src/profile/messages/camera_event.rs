// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

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
    timestamp_ms: Option<u16>,
}

impl CameraEvent {
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
                0 => msg.timestamp_ms = content.one().map(<u16>::from),
                1 => msg.camera_event_type = content.one().map(<crate::profile::enums::CameraEventType>::from),
                2 => msg.camera_file_uuid = content.one().map(<String>::from),
                3 => msg.camera_orientation = content.one().map(<crate::profile::enums::CameraOrientationType>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
