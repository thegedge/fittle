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
pub struct SlaveDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,
}

impl SlaveDevice {
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
                self.manufacturer =field.one().map(|v| {
                    let value = crate::profile::enums::Manufacturer::from(v);
                    value
                })
            },

            1 => {
                self.product =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
