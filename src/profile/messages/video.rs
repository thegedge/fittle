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
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hosting_provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Video {
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
                self.url =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.hosting_provider =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            2 => {
                self.duration =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u32>)(value)
                })
            },

            _ => (),
        }
    }
}
