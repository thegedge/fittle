// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct NmeaSentence {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sentence: Option<String>,
}

impl From<Vec<(u8, Field)>> for NmeaSentence {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sentence = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

