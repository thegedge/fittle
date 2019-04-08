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
pub struct Workout {
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<crate::profile::enums::WorkoutCapabilities>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_name: Option<String>,
}

impl Workout {
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
            4 => {
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            5 => {
                self.capabilities =field.one().map(|v| {
                    let value = crate::profile::enums::WorkoutCapabilities::from(v);
                    value
                })
            },

            6 => {
                self.num_valid_steps =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            8 => {
                self.wkt_name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            11 => {
                self.sub_sport =field.one().map(|v| {
                    let value = crate::profile::enums::SubSport::from(v);
                    value
                })
            },

            14 => {
                self.pool_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            15 => {
                self.pool_length_unit =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
