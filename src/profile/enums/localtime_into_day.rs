use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaltimeIntoDay {
    UnknownValue(u64),
}

impl From<FieldContent> for LocaltimeIntoDay {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                n => LocaltimeIntoDay::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LocaltimeIntoDay to {:?}", field);
        }
    }
}
