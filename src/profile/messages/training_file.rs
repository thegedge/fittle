// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct TrainingFile {
    timestamp: Option<enums::DateTime>,
    type_: Option<enums::File>,
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
}

impl From<Vec<(u8, Field)>> for TrainingFile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.type_ = field.one().map(<enums::File>::from),
                1 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                2 => msg.product = field.one().map(<u16>::from),
                3 => msg.serial_number = field.one().map(<u32>::from),
                4 => msg.time_created = field.one().map(<enums::DateTime>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

