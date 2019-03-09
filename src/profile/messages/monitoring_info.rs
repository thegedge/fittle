// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct MonitoringInfo {
    timestamp: Option<enums::DateTime>,
    local_timestamp: Option<enums::LocalDateTime>,
    activity_type: Option<Vec<enums::ActivityType>>,
    cycles_to_distance: Option<Vec<u16>>,
    cycles_to_calories: Option<Vec<u16>>,
    resting_metabolic_rate: Option<u16>,
}

impl From<Vec<(u8, Field)>> for MonitoringInfo {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                1 => msg.activity_type = field.many().map(|vec| vec.into_iter().map(<enums::ActivityType>::from).collect()),
                3 => msg.cycles_to_distance = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.cycles_to_calories = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.resting_metabolic_rate = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

