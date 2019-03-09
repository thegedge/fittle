// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct TimestampCorrelation {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    system_timestamp: Option<enums::DateTime>,
    fractional_system_timestamp: Option<u16>,
    local_timestamp: Option<enums::LocalDateTime>,
    timestamp_ms: Option<u16>,
    system_timestamp_ms: Option<u16>,
}

impl From<Vec<(u8, Field)>> for TimestampCorrelation {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.system_timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.fractional_system_timestamp = field.one().map(<u16>::from),
                3 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                4 => msg.timestamp_ms = field.one().map(<u16>::from),
                5 => msg.system_timestamp_ms = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

