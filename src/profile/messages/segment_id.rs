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
pub struct SegmentId {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_race_leader: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    delete_status: Option<crate::profile::enums::SegmentDeleteStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selection_type: Option<crate::profile::enums::SegmentSelectionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_primary_key: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
}

impl SegmentId {
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
            0 => {
                self.name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.uuid =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            2 => {
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
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

            5 => {
                self.device_id =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            6 => {
                self.default_race_leader =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            7 => {
                self.delete_status =field.one().map(|v| {
                    let value = crate::profile::enums::SegmentDeleteStatus::from(v);
                    value
                })
            },

            8 => {
                self.selection_type =field.one().map(|v| {
                    let value = crate::profile::enums::SegmentSelectionType::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
