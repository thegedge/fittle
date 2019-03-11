use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    All,
    Cycling,
    FitnessEquipment,
    Generic,
    Running,
    Sedentary,
    Swimming,
    Transition,
    Walking,
    UnknownValue(u64),
}

impl From<FieldContent> for ActivityType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ActivityType::Generic,
                1 => ActivityType::Running,
                2 => ActivityType::Cycling,
                3 => ActivityType::Transition,
                4 => ActivityType::FitnessEquipment,
                5 => ActivityType::Swimming,
                6 => ActivityType::Walking,
                8 => ActivityType::Sedentary,
                254 => ActivityType::All,
                n => ActivityType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ActivityType to {:?}", field);
        }
    }
}
