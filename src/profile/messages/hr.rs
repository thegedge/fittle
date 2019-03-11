// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Hr {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp_12: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filtered_bpm: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time256: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,
}

impl Hr {
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
                0 => msg.fractional_timestamp = content.one().map(<u16>::from),
                1 => msg.time256 = content.one().map(<u8>::from),
                6 => msg.filtered_bpm = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.event_timestamp = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.event_timestamp_12 = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
