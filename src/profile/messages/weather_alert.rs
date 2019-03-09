// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct WeatherAlert {
    timestamp: Option<enums::DateTime>,
    report_id: Option<String>,
    issue_time: Option<enums::DateTime>,
    expire_time: Option<enums::DateTime>,
    severity: Option<enums::WeatherSeverity>,
    type_: Option<enums::WeatherSevereType>,
}

impl From<Vec<(u8, Field)>> for WeatherAlert {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.report_id = field.one().map(<String>::from),
                1 => msg.issue_time = field.one().map(<enums::DateTime>::from),
                2 => msg.expire_time = field.one().map(<enums::DateTime>::from),
                3 => msg.severity = field.one().map(<enums::WeatherSeverity>::from),
                4 => msg.type_ = field.one().map(<enums::WeatherSevereType>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

