// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Hr {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    time256: Option<u8>,
    filtered_bpm: Option<Vec<u8>>,
    event_timestamp: Option<Vec<u32>>,
    event_timestamp_12: Option<Vec<u8>>,
}

impl From<Vec<(u8, Field)>> for Hr {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.time256 = field.one().map(<u8>::from),
                6 => msg.filtered_bpm = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.event_timestamp = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.event_timestamp_12 = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

