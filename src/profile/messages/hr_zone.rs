// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct HrZone {
    message_index: Option<enums::MessageIndex>,
    high_bpm: Option<u8>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for HrZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.high_bpm = field.one().map(<u8>::from),
                2 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

