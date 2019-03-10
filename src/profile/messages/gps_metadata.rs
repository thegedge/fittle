// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct GpsMetadata {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    enhanced_altitude: Option<u32>,
    enhanced_speed: Option<u32>,
    heading: Option<u16>,
    utc_timestamp: Option<enums::DateTime>,
    velocity: Option<Vec<i16>>,
}

impl GpsMetadata {
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
                1 => msg.position_lat = content.one().map(<i32>::from),
                2 => msg.position_long = content.one().map(<i32>::from),
                3 => msg.enhanced_altitude = content.one().map(<u32>::from),
                4 => msg.enhanced_speed = content.one().map(<u32>::from),
                5 => msg.heading = content.one().map(<u16>::from),
                6 => msg.utc_timestamp = content.one().map(<enums::DateTime>::from),
                7 => msg.velocity = content.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

