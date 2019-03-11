use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityClass {
    Athlete,
    Level,
    LevelMax,
    UnknownValue(u64),
}

impl From<FieldContent> for ActivityClass {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                100 => ActivityClass::LevelMax,
                127 => ActivityClass::Level,
                128 => ActivityClass::Athlete,
                n => ActivityClass::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ActivityClass to {:?}", field);
        }
    }
}
