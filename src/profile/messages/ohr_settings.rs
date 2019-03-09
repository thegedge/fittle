// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct OhrSettings {
    timestamp: Option<enums::DateTime>,
    enabled: Option<enums::Switch>,
}

impl From<Vec<(u8, Field)>> for OhrSettings {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.enabled = field.one().map(<enums::Switch>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

