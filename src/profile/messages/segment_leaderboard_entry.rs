// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct SegmentLeaderboardEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<enums::SegmentLeaderboardType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    group_primary_key: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    segment_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id_string: Option<String>,

}

impl SegmentLeaderboardEntry {
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
                0 => msg.name = content.one().map(<String>::from),
                1 => msg.type_ = content.one().map(<enums::SegmentLeaderboardType>::from),
                2 => msg.group_primary_key = content.one().map(<u32>::from),
                3 => msg.activity_id = content.one().map(<u32>::from),
                4 => msg.segment_time = content.one().map(<u32>::from),
                5 => msg.activity_id_string = content.one().map(<String>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

