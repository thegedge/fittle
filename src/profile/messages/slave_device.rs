// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct SlaveDevice {
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
}

impl From<Vec<(u8, Field)>> for SlaveDevice {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                1 => msg.product = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

