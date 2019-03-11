use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeIntoDay {
    UnknownValue(u64),
}

impl From<FieldContent> for TimeIntoDay {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                n => TimeIntoDay::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TimeIntoDay to {:?}", field);
        }
    }
}
