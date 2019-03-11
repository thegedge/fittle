use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutEquipment {
    None,
    SwimFins,
    SwimKickboard,
    SwimPaddles,
    SwimPullBuoy,
    SwimSnorkel,
    UnknownValue(u64),
}

impl From<FieldContent> for WorkoutEquipment {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WorkoutEquipment::None,
                1 => WorkoutEquipment::SwimFins,
                2 => WorkoutEquipment::SwimKickboard,
                3 => WorkoutEquipment::SwimPaddles,
                4 => WorkoutEquipment::SwimPullBuoy,
                5 => WorkoutEquipment::SwimSnorkel,
                n => WorkoutEquipment::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WorkoutEquipment to {:?}", field);
        }
    }
}
