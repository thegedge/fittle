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
pub struct ExdScreenConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<crate::profile::enums::ExdLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_index: Option<u8>,
}

impl ExdScreenConfiguration {
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
                self.screen_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            1 => {
                self.field_count =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.layout =field.one().map(|v| {
                    let value = crate::profile::enums::ExdLayout::from(v);
                    value
                })
            },

            3 => {
                self.screen_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
