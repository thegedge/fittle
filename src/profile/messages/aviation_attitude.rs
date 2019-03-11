// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct AviationAttitude {
    #[serde(skip_serializing_if = "Option::is_none")]
    accel_lateral: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel_normal: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attitude_stage_complete: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roll: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<enums::AttitudeStage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_time: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    turn_rate: Option<Vec<i16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    validity: Option<Vec<enums::AttitudeValidity>>,
}

impl AviationAttitude {
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
                1 => msg.system_time = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                2 => msg.pitch = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                3 => msg.roll = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                4 => msg.accel_lateral = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                5 => msg.accel_normal = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                6 => msg.turn_rate = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                7 => msg.stage = content.many().map(|vec| vec.into_iter().map(<enums::AttitudeStage>::from).collect()),
                8 => msg.attitude_stage_complete = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.track = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                10 => msg.validity = content.many().map(|vec| vec.into_iter().map(<enums::AttitudeValidity>::from).collect()),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
