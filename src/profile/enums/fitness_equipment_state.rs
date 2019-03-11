use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessEquipmentState {
    InUse,
    Paused,
    Ready,
    Unknown,
    UnknownValue(u64),
}

impl From<FieldContent> for FitnessEquipmentState {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => FitnessEquipmentState::Ready,
                1 => FitnessEquipmentState::InUse,
                2 => FitnessEquipmentState::Paused,
                3 => FitnessEquipmentState::Unknown,
                n => FitnessEquipmentState::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FitnessEquipmentState to {:?}", field);
        }
    }
}
