// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct GpsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    utc_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    velocity: Option<Vec<f64>>,
}

impl GpsMetadata {
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
                1 => msg.position_lat = content.one().map(<i32>::from),
                2 => msg.position_long = content.one().map(<i32>::from),
                3 => msg.enhanced_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                4 => msg.enhanced_speed = content.one().map(|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 }),
                5 => msg.heading = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                6 => msg.utc_timestamp = content.one().map(<crate::fields::DateTime>::from),
                7 => msg.velocity = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
