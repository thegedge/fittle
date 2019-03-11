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
pub struct MetZone {
    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fat_calories: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    high_bpm: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,
}

impl MetZone {
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
                1 => msg.high_bpm = content.one().map(<u8>::from),
                2 => msg.calories = content.one().map(<u16>::from),
                3 => msg.fat_calories = content.one().map(<u8>::from),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
