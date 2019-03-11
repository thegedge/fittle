use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalSource {
    Auto,
    Community,
    User,
    UnknownValue(u64),
}

impl From<FieldContent> for GoalSource {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => GoalSource::Auto,
                1 => GoalSource::Community,
                2 => GoalSource::User,
                n => GoalSource::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert GoalSource to {:?}", field);
        }
    }
}
