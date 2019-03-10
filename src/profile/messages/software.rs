// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Software {
    message_index: Option<enums::MessageIndex>,
    version: Option<u16>,
    part_number: Option<String>,
}

impl Software {
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
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                3 => msg.version = content.one().map(<u16>::from),
                5 => msg.part_number = content.one().map(<String>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

