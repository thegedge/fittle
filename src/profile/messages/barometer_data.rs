// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct BarometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    baro_pres: Option<Vec<u32>>,
}

impl From<Vec<(u8, Field)>> for BarometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.baro_pres = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

