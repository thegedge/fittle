// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

#[derive(Debug, Default, Serialize)]
pub struct SegmentFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_race_leader: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    leader_activity_id: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    leader_activity_id_string: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    leader_group_primary_key: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    leader_type: Option<Vec<crate::profile::enums::SegmentLeaderboardType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_primary_key: Option<u32>,
}

impl SegmentFile {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            1 => {
                self.file_uuid =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            3 => {
                self.enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            4 => {
                self.user_profile_primary_key =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            7 => {
                self.leader_type =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::SegmentLeaderboardType::from).collect::<Vec<_>>();
                    value
                })
            },

            8 => {
                self.leader_group_primary_key =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value
                })
            },

            9 => {
                self.leader_activity_id =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value
                })
            },

            10 => {
                self.leader_activity_id_string =field.many().map(|v| {
                    let value = v.into_iter().map(String::from).collect::<Vec<_>>();
                    value
                })
            },

            11 => {
                self.default_race_leader =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            254 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
