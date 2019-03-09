// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct AccelerometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    accel_x: Option<Vec<u16>>,
    accel_y: Option<Vec<u16>>,
    accel_z: Option<Vec<u16>>,
    calibrated_accel_x: Option<Vec<f32>>,
    calibrated_accel_y: Option<Vec<f32>>,
    calibrated_accel_z: Option<Vec<f32>>,
    compressed_calibrated_accel_x: Option<Vec<i16>>,
    compressed_calibrated_accel_y: Option<Vec<i16>>,
    compressed_calibrated_accel_z: Option<Vec<i16>>,
}

impl From<Vec<(u8, Field)>> for AccelerometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.accel_x = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.accel_y = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.accel_z = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_accel_x = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_accel_y = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_accel_z = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                8 => msg.compressed_calibrated_accel_x = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                9 => msg.compressed_calibrated_accel_y = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                10 => msg.compressed_calibrated_accel_z = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

