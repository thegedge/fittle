// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Workout {
    sport: Option<enums::Sport>,
    capabilities: Option<enums::WorkoutCapabilities>,
    num_valid_steps: Option<u16>,
    wkt_name: Option<String>,
    sub_sport: Option<enums::SubSport>,
    pool_length: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
}

impl From<Vec<(u8, Field)>> for Workout {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                4 => msg.sport = field.one().map(<enums::Sport>::from),
                5 => msg.capabilities = field.one().map(<enums::WorkoutCapabilities>::from),
                6 => msg.num_valid_steps = field.one().map(<u16>::from),
                8 => msg.wkt_name = field.one().map(<String>::from),
                11 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                14 => msg.pool_length = field.one().map(<u16>::from),
                15 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

