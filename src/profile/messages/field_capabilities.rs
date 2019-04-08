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
pub struct FieldCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<crate::profile::enums::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_num: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,
}

impl FieldCapabilities {
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
                self.file =field.one().map(|v| {
                    let value = crate::profile::enums::File::from(v);
                    value
                })
            },

            1 => {
                self.mesg_num =field.one().map(|v| {
                    let value = crate::profile::enums::MesgNum::from(v);
                    value
                })
            },

            2 => {
                self.field_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            3 => {
                self.count =field.one().map(|v| {
                    let value = u16::from(v);
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
