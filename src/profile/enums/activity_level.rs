use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityLevel {
    High,
    Low,
    Medium,
    UnknownValue(u64),
}

impl From<FieldContent> for ActivityLevel {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ActivityLevel::Low,
                1 => ActivityLevel::Medium,
                2 => ActivityLevel::High,
                n => ActivityLevel::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ActivityLevel to {:?}", field);
        }
    }
}
