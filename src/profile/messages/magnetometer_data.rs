// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct MagnetometerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_x: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_y: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_z: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_x: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_y: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_z: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sample_time_offset: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,
}

impl MagnetometerData {
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
                0 => msg.timestamp_ms = content.one().map(<u16>::from),
                1 => msg.sample_time_offset = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.mag_x = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.mag_y = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.mag_z = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_mag_x = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_mag_y = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_mag_z = content.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
