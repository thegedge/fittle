use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Active,
    Rest,
    UnknownValue(u64),
}

impl From<FieldContent> for SetType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                0 => SetType::Rest,
                1 => SetType::Active,
                n => SetType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SetType to {:?}", field);
        }
    }
}
