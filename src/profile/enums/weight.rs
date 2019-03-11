use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Weight {
    Calculating,
    UnknownValue(u64),
}

impl From<FieldContent> for Weight {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                65534 => Weight::Calculating,
                n => Weight::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Weight to {:?}", field);
        }
    }
}
