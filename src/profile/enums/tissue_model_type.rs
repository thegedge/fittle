use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TissueModelType {
    Zhl16C,
    UnknownValue(u64),
}

impl From<FieldContent> for TissueModelType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => TissueModelType::Zhl16C,
                n => TissueModelType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TissueModelType to {:?}", field);
        }
    }
}
