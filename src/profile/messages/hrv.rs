// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Hrv {
    time: Option<Vec<u16>>,
}

impl From<Vec<(u8, Field)>> for Hrv {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.time = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

