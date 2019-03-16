// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

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
    total_timer_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::Activity>,
}

impl Activity {
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
                0 => msg.total_timer_time = content.one().map(<u32>::from),
                1 => msg.num_sessions = content.one().map(<u16>::from),
                2 => msg.type_ = content.one().map(<crate::profile::enums::Activity>::from),
                3 => msg.event = content.one().map(<crate::profile::enums::Event>::from),
                4 => msg.event_type = content.one().map(<crate::profile::enums::EventType>::from),
                5 => msg.local_timestamp = content.one().map(<crate::fields::LocalDateTime>::from),
                6 => msg.event_group = content.one().map(<u8>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
