// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Capabilities {
    workouts_supported: Option<enums::WorkoutCapabilities>,
    connectivity_supported: Option<enums::ConnectivityCapabilities>,
}

impl From<Vec<(u8, Field)>> for Capabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                21 => msg.workouts_supported = field.one().map(<enums::WorkoutCapabilities>::from),
                23 => msg.connectivity_supported = field.one().map(<enums::ConnectivityCapabilities>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

