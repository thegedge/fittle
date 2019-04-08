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
pub struct DeveloperDataId {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    application_version: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_data_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer_id: Option<crate::profile::enums::Manufacturer>,
}

impl DeveloperDataId {
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
                self.developer_id =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            1 => {
                self.application_id =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            2 => {
                self.manufacturer_id =field.one().map(|v| {
                    let value = crate::profile::enums::Manufacturer::from(v);
                    value
                })
            },

            3 => {
                self.developer_data_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            4 => {
                self.application_version =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
