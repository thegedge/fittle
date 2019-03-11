use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutCapabilities {
    Cadence,
    Custom,
    Distance,
    Firstbeat,
    FitnessEquipment,
    Grade,
    HeartRate,
    Interval,
    NewLeaf,
    Power,
    Protected,
    Resistance,
    Speed,
    Tcx,
    UnknownValue(u64),
}

impl From<FieldContent> for WorkoutCapabilities {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32z(enum_value) = field {
            match enum_value {
                1 => WorkoutCapabilities::Interval,
                2 => WorkoutCapabilities::Custom,
                4 => WorkoutCapabilities::FitnessEquipment,
                8 => WorkoutCapabilities::Firstbeat,
                16 => WorkoutCapabilities::NewLeaf,
                32 => WorkoutCapabilities::Tcx,
                128 => WorkoutCapabilities::Speed,
                256 => WorkoutCapabilities::HeartRate,
                512 => WorkoutCapabilities::Distance,
                1024 => WorkoutCapabilities::Cadence,
                2048 => WorkoutCapabilities::Power,
                4096 => WorkoutCapabilities::Grade,
                8192 => WorkoutCapabilities::Resistance,
                16384 => WorkoutCapabilities::Protected,
                n => WorkoutCapabilities::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WorkoutCapabilities to {:?}", field);
        }
    }
}
