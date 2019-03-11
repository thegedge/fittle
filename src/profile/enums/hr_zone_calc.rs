use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HrZoneCalc {
    Custom,
    PercentHrr,
    PercentMaxHr,
    UnknownValue(u64),
}

impl From<FieldContent> for HrZoneCalc {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => HrZoneCalc::Custom,
                1 => HrZoneCalc::PercentMaxHr,
                2 => HrZoneCalc::PercentHrr,
                n => HrZoneCalc::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert HrZoneCalc to {:?}", field);
        }
    }
}
