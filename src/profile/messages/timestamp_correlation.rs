// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct TimestampCorrelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_system_timestamp: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<enums::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_timestamp_ms: Option<u16>,

}

impl TimestampCorrelation {
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
                0 => msg.fractional_timestamp = content.one().map(<u16>::from),
                1 => msg.system_timestamp = content.one().map(<enums::DateTime>::from),
                2 => msg.fractional_system_timestamp = content.one().map(<u16>::from),
                3 => msg.local_timestamp = content.one().map(<enums::LocalDateTime>::from),
                4 => msg.timestamp_ms = content.one().map(<u16>::from),
                5 => msg.system_timestamp_ms = content.one().map(<u16>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

