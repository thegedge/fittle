// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct FileCreator {
    software_version: Option<u16>,
    hardware_version: Option<u8>,
}

impl From<Vec<(u8, Field)>> for FileCreator {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.software_version = field.one().map(<u16>::from),
                1 => msg.hardware_version = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

