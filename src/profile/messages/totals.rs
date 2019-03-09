// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Totals {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    timer_time: Option<u32>,
    distance: Option<u32>,
    calories: Option<u32>,
    sport: Option<enums::Sport>,
    elapsed_time: Option<u32>,
    sessions: Option<u16>,
    active_time: Option<u32>,
    sport_index: Option<u8>,
}

impl From<Vec<(u8, Field)>> for Totals {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timer_time = field.one().map(<u32>::from),
                1 => msg.distance = field.one().map(<u32>::from),
                2 => msg.calories = field.one().map(<u32>::from),
                3 => msg.sport = field.one().map(<enums::Sport>::from),
                4 => msg.elapsed_time = field.one().map(<u32>::from),
                5 => msg.sessions = field.one().map(<u16>::from),
                6 => msg.active_time = field.one().map(<u32>::from),
                9 => msg.sport_index = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

