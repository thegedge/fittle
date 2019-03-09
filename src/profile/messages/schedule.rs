// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Schedule {
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
    completed: Option<bool>,
    type_: Option<enums::Schedule>,
    scheduled_time: Option<enums::LocalDateTime>,
}

impl From<Vec<(u8, Field)>> for Schedule {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                1 => msg.product = field.one().map(<u16>::from),
                2 => msg.serial_number = field.one().map(<u32>::from),
                3 => msg.time_created = field.one().map(<enums::DateTime>::from),
                4 => msg.completed = field.one().map(<bool>::from),
                5 => msg.type_ = field.one().map(<enums::Schedule>::from),
                6 => msg.scheduled_time = field.one().map(<enums::LocalDateTime>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

