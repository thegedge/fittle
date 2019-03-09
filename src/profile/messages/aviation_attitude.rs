// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct AviationAttitude {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    system_time: Option<Vec<u32>>,
    pitch: Option<Vec<i16>>,
    roll: Option<Vec<i16>>,
    accel_lateral: Option<Vec<i16>>,
    accel_normal: Option<Vec<i16>>,
    turn_rate: Option<Vec<i16>>,
    stage: Option<Vec<enums::AttitudeStage>>,
    attitude_stage_complete: Option<Vec<u8>>,
    track: Option<Vec<u16>>,
    validity: Option<Vec<enums::AttitudeValidity>>,
}

impl From<Vec<(u8, Field)>> for AviationAttitude {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.system_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                2 => msg.pitch = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                3 => msg.roll = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                4 => msg.accel_lateral = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                5 => msg.accel_normal = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                6 => msg.turn_rate = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                7 => msg.stage = field.many().map(|vec| vec.into_iter().map(<enums::AttitudeStage>::from).collect()),
                8 => msg.attitude_stage_complete = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.track = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                10 => msg.validity = field.many().map(|vec| vec.into_iter().map(<enums::AttitudeValidity>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

