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
pub struct ExdScreenConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<enums::ExdLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_index: Option<u8>,
}

impl ExdScreenConfiguration {
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
                0 => msg.screen_index = content.one().map(<u8>::from),
                1 => msg.field_count = content.one().map(<u8>::from),
                2 => msg.layout = content.one().map(<enums::ExdLayout>::from),
                3 => msg.screen_enabled = content.one().map(<bool>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
