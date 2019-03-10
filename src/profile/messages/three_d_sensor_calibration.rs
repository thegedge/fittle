// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl ThreeDSensorCalibration {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.sensor_type = content.one().map(<enums::SensorType>::from),
                1 => msg.calibration_factor = content.one().map(<u32>::from),
                2 => msg.calibration_divisor = content.one().map(<u32>::from),
                3 => msg.level_shift = content.one().map(<u32>::from),
                4 => msg.offset_cal = content.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                5 => msg.orientation_matrix = content.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

