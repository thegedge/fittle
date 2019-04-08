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
pub struct WorkoutSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_step_index: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_valid_steps: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length_unit: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,
}

impl WorkoutSession {
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
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            1 => {
                self.sub_sport =field.one().map(|v| {
                    let value = crate::profile::enums::SubSport::from(v);
                    value
                })
            },

            2 => {
                self.num_valid_steps =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            3 => {
                self.first_step_index =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            4 => {
                self.pool_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.pool_length_unit =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
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
