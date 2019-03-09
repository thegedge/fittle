// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct CoursePoint {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    type_: Option<enums::CoursePoint>,
    name: Option<String>,
    favorite: Option<bool>,
}

impl From<Vec<(u8, Field)>> for CoursePoint {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.position_lat = field.one().map(<i32>::from),
                3 => msg.position_long = field.one().map(<i32>::from),
                4 => msg.distance = field.one().map(<u32>::from),
                5 => msg.type_ = field.one().map(<enums::CoursePoint>::from),
                6 => msg.name = field.one().map(<String>::from),
                8 => msg.favorite = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

