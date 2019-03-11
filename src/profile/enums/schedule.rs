use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Schedule {
    Course,
    Workout,
    UnknownValue(u64),
}

impl From<FieldContent> for Schedule {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Schedule::Workout,
                1 => Schedule::Course,
                n => Schedule::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Schedule to {:?}", field);
        }
    }
}
