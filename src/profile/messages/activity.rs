// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Activity {
    timestamp: Option<enums::DateTime>,
    total_timer_time: Option<u32>,
    num_sessions: Option<u16>,
    type_: Option<enums::Activity>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    local_timestamp: Option<enums::LocalDateTime>,
    event_group: Option<u8>,
}

impl Activity {
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
                0 => msg.total_timer_time = content.one().map(<u32>::from),
                1 => msg.num_sessions = content.one().map(<u16>::from),
                2 => msg.type_ = content.one().map(<enums::Activity>::from),
                3 => msg.event = content.one().map(<enums::Event>::from),
                4 => msg.event_type = content.one().map(<enums::EventType>::from),
                5 => msg.local_timestamp = content.one().map(<enums::LocalDateTime>::from),
                6 => msg.event_group = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

