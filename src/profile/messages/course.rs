// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Course {
    sport: Option<enums::Sport>,
    name: Option<String>,
    capabilities: Option<enums::CourseCapabilities>,
    sub_sport: Option<enums::SubSport>,
}

impl From<Vec<(u8, Field)>> for Course {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                4 => msg.sport = field.one().map(<enums::Sport>::from),
                5 => msg.name = field.one().map(<String>::from),
                6 => msg.capabilities = field.one().map(<enums::CourseCapabilities>::from),
                7 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

