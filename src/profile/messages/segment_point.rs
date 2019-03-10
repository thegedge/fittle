// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct SegmentPoint {
    message_index: Option<enums::MessageIndex>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    altitude: Option<u16>,
    leader_time: Option<Vec<u32>>,
}

impl SegmentPoint {
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
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                1 => msg.position_lat = content.one().map(<i32>::from),
                2 => msg.position_long = content.one().map(<i32>::from),
                3 => msg.distance = content.one().map(<u32>::from),
                4 => msg.altitude = content.one().map(<u16>::from),
                5 => msg.leader_time = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

