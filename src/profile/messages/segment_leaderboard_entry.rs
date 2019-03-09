// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct SegmentLeaderboardEntry {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    type_: Option<enums::SegmentLeaderboardType>,
    group_primary_key: Option<u32>,
    activity_id: Option<u32>,
    segment_time: Option<u32>,
    activity_id_string: Option<String>,
}

impl From<Vec<(u8, Field)>> for SegmentLeaderboardEntry {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.type_ = field.one().map(<enums::SegmentLeaderboardType>::from),
                2 => msg.group_primary_key = field.one().map(<u32>::from),
                3 => msg.activity_id = field.one().map(<u32>::from),
                4 => msg.segment_time = field.one().map(<u32>::from),
                5 => msg.activity_id_string = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

