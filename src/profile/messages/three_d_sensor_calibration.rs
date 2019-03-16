// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct ThreeDSensorCalibration {
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration_divisor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibration_factor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    level_shift: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset_cal: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orientation_matrix: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sensor_type: Option<crate::profile::enums::SensorType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl ThreeDSensorCalibration {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.sensor_type = content.one().map(<crate::profile::enums::SensorType>::from),
                1 => msg.calibration_factor = content.one().map(<u32>::from),
                2 => msg.calibration_divisor = content.one().map(<u32>::from),
                3 => msg.level_shift = content.one().map(<u32>::from),
                4 => msg.offset_cal = content.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                5 => msg.orientation_matrix = content.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
