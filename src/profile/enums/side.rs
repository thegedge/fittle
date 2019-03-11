use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Left,
    Right,
    UnknownValue(u64),
}

impl From<FieldContent> for Side {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Side::Right,
                1 => Side::Left,
                n => Side::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Side to {:?}", field);
        }
    }
}
