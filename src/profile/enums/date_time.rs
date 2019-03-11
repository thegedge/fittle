use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DateTime {
    Min,
    UnknownValue(u64),
}

impl From<FieldContent> for DateTime {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                268435456 => DateTime::Min,
                n => DateTime::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DateTime to {:?}", field);
        }
    }
}
