use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionTrigger {
    ActivityEnd,
    AutoMultiSport,
    FitnessEquipment,
    Manual,
    UnknownValue(u64),
}

impl From<FieldContent> for SessionTrigger {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SessionTrigger::ActivityEnd,
                1 => SessionTrigger::Manual,
                2 => SessionTrigger::AutoMultiSport,
                3 => SessionTrigger::FitnessEquipment,
                n => SessionTrigger::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SessionTrigger to {:?}", field);
        }
    }
}
