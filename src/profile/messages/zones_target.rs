// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ZonesTarget {
    max_heart_rate: Option<u8>,
    threshold_heart_rate: Option<u8>,
    functional_threshold_power: Option<u16>,
    hr_calc_type: Option<enums::HrZoneCalc>,
    pwr_calc_type: Option<enums::PwrZoneCalc>,
}

impl From<Vec<(u8, Field)>> for ZonesTarget {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                1 => msg.max_heart_rate = field.one().map(<u8>::from),
                2 => msg.threshold_heart_rate = field.one().map(<u8>::from),
                3 => msg.functional_threshold_power = field.one().map(<u16>::from),
                5 => msg.hr_calc_type = field.one().map(<enums::HrZoneCalc>::from),
                7 => msg.pwr_calc_type = field.one().map(<enums::PwrZoneCalc>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

