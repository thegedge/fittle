// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl SegmentFile {
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
                1 => msg.file_uuid = content.one().map(<String>::from),
                3 => msg.enabled = content.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = content.one().map(<u32>::from),
                7 => msg.leader_type = content.many().map(|vec| vec.into_iter().map(<enums::SegmentLeaderboardType>::from).collect()),
                8 => msg.leader_group_primary_key = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                9 => msg.leader_activity_id = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.leader_activity_id_string = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                11 => msg.default_race_leader = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

