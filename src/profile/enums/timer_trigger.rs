use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimerTrigger {
    Auto,
    FitnessEquipment,
    Manual,
    UnknownValue(u64),
}

impl From<FieldContent> for TimerTrigger {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => TimerTrigger::Manual,
                1 => TimerTrigger::Auto,
                2 => TimerTrigger::FitnessEquipment,
                n => TimerTrigger::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TimerTrigger to {:?}", field);
        }
    }
}
