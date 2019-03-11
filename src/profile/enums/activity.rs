use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    AutoMultiSport,
    Manual,
    UnknownValue(u64),
}

impl From<FieldContent> for Activity {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Activity::Manual,
                1 => Activity::AutoMultiSport,
                n => Activity::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Activity to {:?}", field);
        }
    }
}
