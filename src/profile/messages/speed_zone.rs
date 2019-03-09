// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct SpeedZone {
    message_index: Option<enums::MessageIndex>,
    high_value: Option<u16>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for SpeedZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.high_value = field.one().map(<u16>::from),
                1 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

