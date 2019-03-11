use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BacklightTimeout {
    Infinite,
    UnknownValue(u64),
}

impl From<FieldContent> for BacklightTimeout {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                0 => BacklightTimeout::Infinite,
                n => BacklightTimeout::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BacklightTimeout to {:?}", field);
        }
    }
}
