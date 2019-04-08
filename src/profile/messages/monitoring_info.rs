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
pub struct MonitoringInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<Vec<crate::profile::enums::ActivityType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_to_calories: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_to_distance: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<crate::fields::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resting_metabolic_rate: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl MonitoringInfo {
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
                self.local_timestamp =field.one().map(|v| {
                    let value = crate::fields::LocalDateTime::from(v);
                    value
                })
            },

            1 => {
                self.activity_type =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::ActivityType::from).collect::<Vec<_>>();
                    value
                })
            },

            3 => {
                self.cycles_to_distance =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 5000.0 - 0.0 }).collect()
                })
            },

            4 => {
                self.cycles_to_calories =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 5000.0 - 0.0 }).collect()
                })
            },

            5 => {
                self.resting_metabolic_rate =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
