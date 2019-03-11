use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayPower {
    PercentFtp,
    Watts,
    UnknownValue(u64),
}

impl From<FieldContent> for DisplayPower {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DisplayPower::Watts,
                1 => DisplayPower::PercentFtp,
                n => DisplayPower::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DisplayPower to {:?}", field);
        }
    }
}
