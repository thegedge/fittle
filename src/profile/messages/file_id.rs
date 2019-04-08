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
pub struct FileId {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_created: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::File>,
}

impl FileId {
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
                self.manufacturer =field.one().map(|v| {
                    let value = crate::profile::enums::Manufacturer::from(v);
                    value
                })
            },

            2 => {
                self.product =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            3 => {
                self.serial_number =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            4 => {
                self.time_created =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            5 => {
                self.number =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            8 => {
                self.product_name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
