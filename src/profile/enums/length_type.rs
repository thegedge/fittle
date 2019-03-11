use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LengthType {
    Active,
    Idle,
    UnknownValue(u64),
}

impl From<FieldContent> for LengthType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => LengthType::Idle,
                1 => LengthType::Active,
                n => LengthType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LengthType to {:?}", field);
        }
    }
}
