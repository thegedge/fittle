// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct VideoClip {
    clip_number: Option<u16>,
    start_timestamp: Option<enums::DateTime>,
    start_timestamp_ms: Option<u16>,
    end_timestamp: Option<enums::DateTime>,
    end_timestamp_ms: Option<u16>,
    clip_start: Option<u32>,
    clip_end: Option<u32>,
}

impl From<Vec<(u8, Field)>> for VideoClip {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.clip_number = field.one().map(<u16>::from),
                1 => msg.start_timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.start_timestamp_ms = field.one().map(<u16>::from),
                3 => msg.end_timestamp = field.one().map(<enums::DateTime>::from),
                4 => msg.end_timestamp_ms = field.one().map(<u16>::from),
                6 => msg.clip_start = field.one().map(<u32>::from),
                7 => msg.clip_end = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

