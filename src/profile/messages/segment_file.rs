// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct SegmentFile {
    message_index: Option<enums::MessageIndex>,
    file_uuid: Option<String>,
    enabled: Option<bool>,
    user_profile_primary_key: Option<u32>,
    leader_type: Option<Vec<enums::SegmentLeaderboardType>>,
    leader_group_primary_key: Option<Vec<u32>>,
    leader_activity_id: Option<Vec<u32>>,
    leader_activity_id_string: Option<Vec<String>>,
    default_race_leader: Option<u8>,
}

impl From<Vec<(u8, Field)>> for SegmentFile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.file_uuid = field.one().map(<String>::from),
                3 => msg.enabled = field.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = field.one().map(<u32>::from),
                7 => msg.leader_type = field.many().map(|vec| vec.into_iter().map(<enums::SegmentLeaderboardType>::from).collect()),
                8 => msg.leader_group_primary_key = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                9 => msg.leader_activity_id = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.leader_activity_id_string = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                11 => msg.default_race_leader = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

