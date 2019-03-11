use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutolapTrigger {
    Distance,
    Off,
    PositionLap,
    PositionMarked,
    PositionStart,
    PositionWaypoint,
    Time,
    UnknownValue(u64),
}

impl From<FieldContent> for AutolapTrigger {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => AutolapTrigger::Time,
                1 => AutolapTrigger::Distance,
                2 => AutolapTrigger::PositionStart,
                3 => AutolapTrigger::PositionLap,
                4 => AutolapTrigger::PositionWaypoint,
                5 => AutolapTrigger::PositionMarked,
                6 => AutolapTrigger::Off,
                n => AutolapTrigger::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AutolapTrigger to {:?}", field);
        }
    }
}
