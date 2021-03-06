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
pub struct Totals {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sessions: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timer_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl Totals {
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
                self.timer_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u32>)(value)
                })
            },

            1 => {
                self.distance =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u32>)(value)
                })
            },

            2 => {
                self.calories =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u32>)(value)
                })
            },

            3 => {
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            4 => {
                self.elapsed_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u32>)(value)
                })
            },

            5 => {
                self.sessions =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            6 => {
                self.active_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u32>)(value)
                })
            },

            9 => {
                self.sport_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
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
