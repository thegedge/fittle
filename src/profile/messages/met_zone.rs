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
pub struct MetZone {
    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fat_calories: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    high_bpm: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,
}

impl MetZone {
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
            1 => {
                self.high_bpm =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            3 => {
                self.fat_calories =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
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
