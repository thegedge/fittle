// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Sport {
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for Sport {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                3 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

