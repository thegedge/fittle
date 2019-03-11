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
pub struct Software {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    part_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u16>,
}

impl Software {
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
                3 => msg.version = content.one().map(<u16>::from),
                5 => msg.part_number = content.one().map(<String>::from),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
