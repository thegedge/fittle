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
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<crate::fields::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_sessions: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_timer_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::Activity>,
}

impl Activity {
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
                self.total_timer_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            1 => {
                self.num_sessions =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            2 => {
                self.type_ =field.one().map(|v| {
                    let value = crate::profile::enums::Activity::from(v);
                    value
                })
            },

            3 => {
                self.event =field.one().map(|v| {
                    let value = crate::profile::enums::Event::from(v);
                    value
                })
            },

            4 => {
                self.event_type =field.one().map(|v| {
                    let value = crate::profile::enums::EventType::from(v);
                    value
                })
            },

            5 => {
                self.local_timestamp =field.one().map(|v| {
                    let value = crate::fields::LocalDateTime::from(v);
                    value
                })
            },

            6 => {
                self.event_group =field.one().map(|v| {
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

            _ => (),
        }
    }
}
