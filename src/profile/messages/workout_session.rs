// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct WorkoutSession {
    message_index: Option<enums::MessageIndex>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    num_valid_steps: Option<u16>,
    first_step_index: Option<u16>,
    pool_length: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
}

impl From<Vec<(u8, Field)>> for WorkoutSession {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                2 => msg.num_valid_steps = field.one().map(<u16>::from),
                3 => msg.first_step_index = field.one().map(<u16>::from),
                4 => msg.pool_length = field.one().map(<u16>::from),
                5 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

