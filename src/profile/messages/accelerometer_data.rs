// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct AccelerometerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    accel_x: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel_y: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel_z: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_accel_x: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_accel_y: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_accel_z: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_calibrated_accel_x: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_calibrated_accel_y: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_calibrated_accel_z: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sample_time_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,
}

impl AccelerometerData {
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
                0 => msg.timestamp_ms = content.one().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u16>((<u16>::from)(v))),
                1 => msg.sample_time_offset = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u16>((<u16>::from)(v))).collect()),
                2 => msg.accel_x = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.accel_y = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.accel_z = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_accel_x = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_accel_y = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_accel_z = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                8 => msg.compressed_calibrated_accel_x = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                9 => msg.compressed_calibrated_accel_y = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                10 => msg.compressed_calibrated_accel_z = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
