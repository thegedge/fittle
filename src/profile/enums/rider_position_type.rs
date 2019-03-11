use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RiderPositionType {
    Seated,
    Standing,
    TransitionToSeated,
    TransitionToStanding,
    UnknownValue(u64),
}

impl From<FieldContent> for RiderPositionType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => RiderPositionType::Seated,
                1 => RiderPositionType::Standing,
                2 => RiderPositionType::TransitionToSeated,
                3 => RiderPositionType::TransitionToStanding,
                n => RiderPositionType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert RiderPositionType to {:?}", field);
        }
    }
}
