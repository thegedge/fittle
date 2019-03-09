// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ThreeDSensorCalibration {
    timestamp: Option<enums::DateTime>,
    sensor_type: Option<enums::SensorType>,
    calibration_factor: Option<u32>,
    calibration_divisor: Option<u32>,
    level_shift: Option<u32>,
    offset_cal: Option<Vec<i32>>,
    orientation_matrix: Option<Vec<i32>>,
}

impl From<Vec<(u8, Field)>> for ThreeDSensorCalibration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.sensor_type = field.one().map(<enums::SensorType>::from),
                1 => msg.calibration_factor = field.one().map(<u32>::from),
                2 => msg.calibration_divisor = field.one().map(<u32>::from),
                3 => msg.level_shift = field.one().map(<u32>::from),
                4 => msg.offset_cal = field.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                5 => msg.orientation_matrix = field.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

