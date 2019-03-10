// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl AccelerometerData {
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
                0 => msg.timestamp_ms = content.one().map(<u16>::from),
                1 => msg.sample_time_offset = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.accel_x = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.accel_y = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.accel_z = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_accel_x = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_accel_y = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_accel_z = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                8 => msg.compressed_calibrated_accel_x = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                9 => msg.compressed_calibrated_accel_y = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                10 => msg.compressed_calibrated_accel_z = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

