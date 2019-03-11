use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerPhaseType {
    PowerPhaseArcLength,
    PowerPhaseCenter,
    PowerPhaseEndAngle,
    PowerPhaseStartAngle,
    UnknownValue(u64),
}

impl From<FieldContent> for PowerPhaseType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => PowerPhaseType::PowerPhaseStartAngle,
                1 => PowerPhaseType::PowerPhaseEndAngle,
                2 => PowerPhaseType::PowerPhaseArcLength,
                3 => PowerPhaseType::PowerPhaseCenter,
                n => PowerPhaseType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert PowerPhaseType to {:?}", field);
        }
    }
}
