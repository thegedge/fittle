// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Totals {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<u32>,

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
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.timer_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))),
                1 => msg.distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u32>((<u32>::from)(v))),
                2 => msg.calories = content.one().map(<u32>::from),
                3 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                4 => msg.elapsed_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))),
                5 => msg.sessions = content.one().map(<u16>::from),
                6 => msg.active_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))),
                9 => msg.sport_index = content.one().map(<u8>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
