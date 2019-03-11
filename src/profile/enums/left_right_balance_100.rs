use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LeftRightBalance100 {
    Mask,
    Right,
    UnknownValue(u64),
}

impl From<FieldContent> for LeftRightBalance100 {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                16383 => LeftRightBalance100::Mask,
                32768 => LeftRightBalance100::Right,
                n => LeftRightBalance100::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LeftRightBalance100 to {:?}", field);
        }
    }
}
