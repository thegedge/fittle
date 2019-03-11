use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalDateTime {
    Min,
    UnknownValue(u64),
}

impl From<FieldContent> for LocalDateTime {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                268435456 => LocalDateTime::Min,
                n => LocalDateTime::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LocalDateTime to {:?}", field);
        }
    }
}
