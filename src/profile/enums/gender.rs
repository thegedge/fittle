use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,
    Male,
    UnknownValue(u64),
}

impl From<FieldContent> for Gender {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Gender::Female,
                1 => Gender::Male,
                n => Gender::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Gender to {:?}", field);
        }
    }
}
