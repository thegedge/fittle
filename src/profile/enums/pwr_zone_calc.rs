use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PwrZoneCalc {
    Custom,
    PercentFtp,
    UnknownValue(u64),
}

impl From<FieldContent> for PwrZoneCalc {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => PwrZoneCalc::Custom,
                1 => PwrZoneCalc::PercentFtp,
                n => PwrZoneCalc::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert PwrZoneCalc to {:?}", field);
        }
    }
}
