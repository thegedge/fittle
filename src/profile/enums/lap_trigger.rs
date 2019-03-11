use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LapTrigger {
    Distance,
    FitnessEquipment,
    Manual,
    PositionLap,
    PositionMarked,
    PositionStart,
    PositionWaypoint,
    SessionEnd,
    Time,
    UnknownValue(u64),
}

impl From<FieldContent> for LapTrigger {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => LapTrigger::Manual,
                1 => LapTrigger::Time,
                2 => LapTrigger::Distance,
                3 => LapTrigger::PositionStart,
                4 => LapTrigger::PositionLap,
                5 => LapTrigger::PositionWaypoint,
                6 => LapTrigger::PositionMarked,
                7 => LapTrigger::SessionEnd,
                8 => LapTrigger::FitnessEquipment,
                n => LapTrigger::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LapTrigger to {:?}", field);
        }
    }
}
