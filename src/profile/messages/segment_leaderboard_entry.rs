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
pub struct SegmentLeaderboardEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    group_primary_key: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    segment_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::SegmentLeaderboardType>,
}

impl SegmentLeaderboardEntry {
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
                self.type_ =field.one().map(|v| {
                    let value = crate::profile::enums::SegmentLeaderboardType::from(v);
                    value
                })
            },

            2 => {
                self.group_primary_key =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            3 => {
                self.activity_id =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            4 => {
                self.segment_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.activity_id_string =field.one().map(|v| {
                    let value = String::from(v);
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
