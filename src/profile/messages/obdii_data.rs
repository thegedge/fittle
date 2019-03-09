// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ObdiiData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    time_offset: Option<Vec<u16>>,
    pid: Option<u8>,
    raw_data: Option<Vec<u8>>,
    pid_data_size: Option<Vec<u8>>,
    system_time: Option<Vec<u32>>,
    start_timestamp: Option<enums::DateTime>,
    start_timestamp_ms: Option<u16>,
}

impl From<Vec<(u8, Field)>> for ObdiiData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.pid = field.one().map(<u8>::from),
                3 => msg.raw_data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                4 => msg.pid_data_size = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                5 => msg.system_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                6 => msg.start_timestamp = field.one().map(<enums::DateTime>::from),
                7 => msg.start_timestamp_ms = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

