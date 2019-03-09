// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for Event {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.data16 = field.one().map(<u16>::from),
                3 => msg.data = field.one().map(<u32>::from),
                4 => msg.event_group = field.one().map(<u8>::from),
                7 => msg.score = field.one().map(<u16>::from),
                8 => msg.opponent_score = field.one().map(<u16>::from),
                9 => msg.front_gear_num = field.one().map(<u8>::from),
                10 => msg.front_gear = field.one().map(<u8>::from),
                11 => msg.rear_gear_num = field.one().map(<u8>::from),
                12 => msg.rear_gear = field.one().map(<u8>::from),
                13 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

