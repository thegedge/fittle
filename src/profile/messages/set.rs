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
pub struct Set {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Vec<crate::profile::enums::ExerciseCategory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    category_subtype: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repetitions: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    set_type: Option<crate::profile::enums::SetType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_display_unit: Option<crate::profile::enums::FitBaseUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<crate::profile::enums::MessageIndex>,
}

impl Set {
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
                self.duration =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            3 => {
                self.repetitions =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            4 => {
                self.weight =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 16.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.set_type =field.one().map(|v| {
                    let value = crate::profile::enums::SetType::from(v);
                    value
                })
            },

            6 => {
                self.start_time =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            7 => {
                self.category =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::ExerciseCategory::from).collect::<Vec<_>>();
                    value
                })
            },

            8 => {
                self.category_subtype =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            9 => {
                self.weight_display_unit =field.one().map(|v| {
                    let value = crate::profile::enums::FitBaseUnit::from(v);
                    value
                })
            },

            10 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            11 => {
                self.wkt_step_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            254 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
