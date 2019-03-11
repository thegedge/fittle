use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeftRightBalance {
    Mask,
    Right,
    UnknownValue(u64),
}

impl From<FieldContent> for LeftRightBalance {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                127 => LeftRightBalance::Mask,
                128 => LeftRightBalance::Right,
                n => LeftRightBalance::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LeftRightBalance to {:?}", field);
        }
    }
}
