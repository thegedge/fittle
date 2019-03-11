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
pub struct ObdiiData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pid_data_size: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    raw_data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_time: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_offset: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,
}

impl ObdiiData {
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
                1 => msg.time_offset = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.pid = content.one().map(<u8>::from),
                3 => msg.raw_data = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                4 => msg.pid_data_size = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                5 => msg.system_time = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                6 => msg.start_timestamp = content.one().map(<enums::DateTime>::from),
                7 => msg.start_timestamp_ms = content.one().map(<u16>::from),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
