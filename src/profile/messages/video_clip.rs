// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct VideoClip {
    #[serde(skip_serializing_if = "Option::is_none")]
    clip_end: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clip_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clip_start: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp_ms: Option<u16>,
}

impl VideoClip {
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
                0 => msg.clip_number = content.one().map(<u16>::from),
                1 => msg.start_timestamp = content.one().map(<crate::fields::DateTime>::from),
                2 => msg.start_timestamp_ms = content.one().map(<u16>::from),
                3 => msg.end_timestamp = content.one().map(<crate::fields::DateTime>::from),
                4 => msg.end_timestamp_ms = content.one().map(<u16>::from),
                6 => msg.clip_start = content.one().map(<u32>::from),
                7 => msg.clip_end = content.one().map(<u32>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
