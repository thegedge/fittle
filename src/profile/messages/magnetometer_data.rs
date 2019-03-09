// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct MagnetometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    mag_x: Option<Vec<u16>>,
    mag_y: Option<Vec<u16>>,
    mag_z: Option<Vec<u16>>,
    calibrated_mag_x: Option<Vec<f32>>,
    calibrated_mag_y: Option<Vec<f32>>,
    calibrated_mag_z: Option<Vec<f32>>,
}

impl From<Vec<(u8, Field)>> for MagnetometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.mag_x = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.mag_y = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.mag_z = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_mag_x = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_mag_y = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_mag_z = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

