use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutPower {
    WattsOffset,
    UnknownValue(u64),
}

impl From<FieldContent> for WorkoutPower {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                1000 => WorkoutPower::WattsOffset,
                n => WorkoutPower::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WorkoutPower to {:?}", field);
        }
    }
}
