// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct AviationAttitude {
    #[serde(skip_serializing_if = "Option::is_none")]
    accel_lateral: Option<Vec<crate::fields::Acceleration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel_normal: Option<Vec<crate::fields::Acceleration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attitude_stage_complete: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roll: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<crate::profile::enums::AttitudeStage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_time: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    turn_rate: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    validity: Option<Vec<crate::profile::enums::AttitudeValidity>>,
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
                0 => msg.timestamp_ms = content.one().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u16>((<u16>::from)(v))),
                1 => msg.system_time = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u32>((<u32>::from)(v))).collect()),
                2 => msg.pitch = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<i16>::from(v)) / 10430.0 - 0.0 }).collect()),
                3 => msg.roll = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<i16>::from(v)) / 10430.0 - 0.0 }).collect()),
                4 => msg.accel_lateral = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Acceleration::new::<uom::si::acceleration::meter_per_second_squared, f64>((|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 })(v))).collect()),
                5 => msg.accel_normal = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Acceleration::new::<uom::si::acceleration::meter_per_second_squared, f64>((|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 })(v))).collect()),
                6 => msg.turn_rate = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Frequency::new::<uom::si::frequency::hertz, f64>((|v| { <f64>::from(<i16>::from(v)) / 1024.0 - 0.0 })(v))).collect()),
                7 => msg.stage = content.many().map(|vec| vec.into_iter().map(<crate::profile::enums::AttitudeStage>::from).collect()),
                8 => msg.attitude_stage_complete = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.track = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10430.0 - 0.0 }).collect()),
                10 => msg.validity = content.many().map(|vec| vec.into_iter().map(<crate::profile::enums::AttitudeValidity>::from).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
