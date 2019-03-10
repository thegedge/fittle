// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Event {
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    data16: Option<u16>,
    data: Option<u32>,
    event_group: Option<u8>,
    score: Option<u16>,
    opponent_score: Option<u16>,
    front_gear_num: Option<u8>,
    front_gear: Option<u8>,
    rear_gear_num: Option<u8>,
    rear_gear: Option<u8>,
    device_index: Option<enums::DeviceIndex>,
}

impl Event {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.event = content.one().map(<enums::Event>::from),
                1 => msg.event_type = content.one().map(<enums::EventType>::from),
                2 => msg.data16 = content.one().map(<u16>::from),
                3 => msg.data = content.one().map(<u32>::from),
                4 => msg.event_group = content.one().map(<u8>::from),
                7 => msg.score = content.one().map(<u16>::from),
                8 => msg.opponent_score = content.one().map(<u16>::from),
                9 => msg.front_gear_num = content.one().map(<u8>::from),
                10 => msg.front_gear = content.one().map(<u8>::from),
                11 => msg.rear_gear_num = content.one().map(<u8>::from),
                12 => msg.rear_gear = content.one().map(<u8>::from),
                13 => msg.device_index = content.one().map(<enums::DeviceIndex>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

