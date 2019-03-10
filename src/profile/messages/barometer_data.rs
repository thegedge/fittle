// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct BarometerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sample_time_offset: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    baro_pres: Option<Vec<u32>>,

}

impl BarometerData {
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
                1 => msg.sample_time_offset = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.baro_pres = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

