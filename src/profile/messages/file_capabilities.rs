// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

#[derive(Debug, Default, Serialize)]
pub struct FileCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<crate::profile::enums::FileFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::File>,
}

impl FileCapabilities {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.type_ =field.one().map(|v| {
                    let value = crate::profile::enums::File::from(v);
                    value
                })
            },

            1 => {
                self.flags =field.one().map(|v| {
                    let value = crate::profile::enums::FileFlags::from(v);
                    value
                })
            },

            2 => {
                self.directory =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            3 => {
                self.max_count =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            4 => {
                self.max_size =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            254 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
