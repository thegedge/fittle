// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for Activity {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.total_timer_time = field.one().map(<u32>::from),
                1 => msg.num_sessions = field.one().map(<u16>::from),
                2 => msg.type_ = field.one().map(<enums::Activity>::from),
                3 => msg.event = field.one().map(<enums::Event>::from),
                4 => msg.event_type = field.one().map(<enums::EventType>::from),
                5 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                6 => msg.event_group = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

