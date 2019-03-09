// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct BloodPressure {
    timestamp: Option<enums::DateTime>,
    systolic_pressure: Option<u16>,
    diastolic_pressure: Option<u16>,
    mean_arterial_pressure: Option<u16>,
    map_3_sample_mean: Option<u16>,
    map_morning_values: Option<u16>,
    map_evening_values: Option<u16>,
    heart_rate: Option<u8>,
    heart_rate_type: Option<enums::HrType>,
    status: Option<enums::BpStatus>,
    user_profile_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for BloodPressure {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.systolic_pressure = field.one().map(<u16>::from),
                1 => msg.diastolic_pressure = field.one().map(<u16>::from),
                2 => msg.mean_arterial_pressure = field.one().map(<u16>::from),
                3 => msg.map_3_sample_mean = field.one().map(<u16>::from),
                4 => msg.map_morning_values = field.one().map(<u16>::from),
                5 => msg.map_evening_values = field.one().map(<u16>::from),
                6 => msg.heart_rate = field.one().map(<u8>::from),
                7 => msg.heart_rate_type = field.one().map(<enums::HrType>::from),
                8 => msg.status = field.one().map(<enums::BpStatus>::from),
                9 => msg.user_profile_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

