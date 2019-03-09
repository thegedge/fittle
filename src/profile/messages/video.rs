// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Video {
    url: Option<String>,
    hosting_provider: Option<String>,
    duration: Option<u32>,
}

impl From<Vec<(u8, Field)>> for Video {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.url = field.one().map(<String>::from),
                1 => msg.hosting_provider = field.one().map(<String>::from),
                2 => msg.duration = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

