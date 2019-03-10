// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Totals {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timer_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sessions: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport_index: Option<u8>,

}

impl Totals {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.timer_time = content.one().map(<u32>::from),
                1 => msg.distance = content.one().map(<u32>::from),
                2 => msg.calories = content.one().map(<u32>::from),
                3 => msg.sport = content.one().map(<enums::Sport>::from),
                4 => msg.elapsed_time = content.one().map(<u32>::from),
                5 => msg.sessions = content.one().map(<u16>::from),
                6 => msg.active_time = content.one().map(<u32>::from),
                9 => msg.sport_index = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

