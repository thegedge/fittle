// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl Event {
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
                0 => msg.event = content.one().map(<crate::profile::enums::Event>::from),
                1 => msg.event_type = content.one().map(<crate::profile::enums::EventType>::from),
                2 => msg.data16 = content.one().map(<u16>::from),
                3 => msg.data = content.one().map(<u32>::from),
                4 => msg.event_group = content.one().map(<u8>::from),
                7 => msg.score = content.one().map(<u16>::from),
                8 => msg.opponent_score = content.one().map(<u16>::from),
                9 => msg.front_gear_num = content.one().map(<u8>::from),
                10 => msg.front_gear = content.one().map(<u8>::from),
                11 => msg.rear_gear_num = content.one().map(<u8>::from),
                12 => msg.rear_gear = content.one().map(<u8>::from),
                13 => msg.device_index = content.one().map(<crate::profile::enums::DeviceIndex>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
