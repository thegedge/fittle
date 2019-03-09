// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Software {
    message_index: Option<enums::MessageIndex>,
    version: Option<u16>,
    part_number: Option<String>,
}

impl From<Vec<(u8, Field)>> for Software {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                3 => msg.version = field.one().map(<u16>::from),
                5 => msg.part_number = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

