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
pub struct Goal {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<crate::profile::enums::GoalRecurrence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence_value: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<crate::profile::enums::GoalSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::Goal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
}

impl Goal {
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
                self.start_date =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            3 => {
                self.end_date =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            4 => {
                self.type_ =field.one().map(|v| {
                    let value = crate::profile::enums::Goal::from(v);
                    value
                })
            },

            5 => {
                self.value =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            6 => {
                self.repeat =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            7 => {
                self.target_value =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            8 => {
                self.recurrence =field.one().map(|v| {
                    let value = crate::profile::enums::GoalRecurrence::from(v);
                    value
                })
            },

            9 => {
                self.recurrence_value =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            10 => {
                self.enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            11 => {
                self.source =field.one().map(|v| {
                    let value = crate::profile::enums::GoalSource::from(v);
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
