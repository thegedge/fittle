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
pub struct DiveAlarm {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_type: Option<crate::profile::enums::DiveAlarmType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_types: Option<Vec<crate::profile::enums::SubSport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<crate::profile::enums::Tone>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<crate::fields::Time>,
}

impl DiveAlarm {
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
                self.depth =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            1 => {
                self.time =field.one().map(|v| {
                    let value = i32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            2 => {
                self.enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            3 => {
                self.alarm_type =field.one().map(|v| {
                    let value = crate::profile::enums::DiveAlarmType::from(v);
                    value
                })
            },

            4 => {
                self.sound =field.one().map(|v| {
                    let value = crate::profile::enums::Tone::from(v);
                    value
                })
            },

            5 => {
                self.dive_types =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::SubSport::from).collect::<Vec<_>>();
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
