use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutHr {
    BpmOffset,
    UnknownValue(u64),
}

impl From<FieldContent> for WorkoutHr {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                100 => WorkoutHr::BpmOffset,
                n => WorkoutHr::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WorkoutHr to {:?}", field);
        }
    }
}
