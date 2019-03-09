// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct SegmentPoint {
    message_index: Option<enums::MessageIndex>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    altitude: Option<u16>,
    leader_time: Option<Vec<u32>>,
}

impl From<Vec<(u8, Field)>> for SegmentPoint {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.position_lat = field.one().map(<i32>::from),
                2 => msg.position_long = field.one().map(<i32>::from),
                3 => msg.distance = field.one().map(<u32>::from),
                4 => msg.altitude = field.one().map(<u16>::from),
                5 => msg.leader_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

