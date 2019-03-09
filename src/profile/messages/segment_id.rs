// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for SegmentId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.uuid = field.one().map(<String>::from),
                2 => msg.sport = field.one().map(<enums::Sport>::from),
                3 => msg.enabled = field.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = field.one().map(<u32>::from),
                5 => msg.device_id = field.one().map(<u32>::from),
                6 => msg.default_race_leader = field.one().map(<u8>::from),
                7 => msg.delete_status = field.one().map(<enums::SegmentDeleteStatus>::from),
                8 => msg.selection_type = field.one().map(<enums::SegmentSelectionType>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

