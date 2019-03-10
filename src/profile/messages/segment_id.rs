// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct SegmentId {
    name: Option<String>,
    uuid: Option<String>,
    sport: Option<enums::Sport>,
    enabled: Option<bool>,
    user_profile_primary_key: Option<u32>,
    device_id: Option<u32>,
    default_race_leader: Option<u8>,
    delete_status: Option<enums::SegmentDeleteStatus>,
    selection_type: Option<enums::SegmentSelectionType>,
}

impl SegmentId {
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
                0 => msg.name = content.one().map(<String>::from),
                1 => msg.uuid = content.one().map(<String>::from),
                2 => msg.sport = content.one().map(<enums::Sport>::from),
                3 => msg.enabled = content.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = content.one().map(<u32>::from),
                5 => msg.device_id = content.one().map(<u32>::from),
                6 => msg.default_race_leader = content.one().map(<u8>::from),
                7 => msg.delete_status = content.one().map(<enums::SegmentDeleteStatus>::from),
                8 => msg.selection_type = content.one().map(<enums::SegmentSelectionType>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

