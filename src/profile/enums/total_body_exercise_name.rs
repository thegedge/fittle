use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TotalBodyExerciseName {
    Burpee,
    BurpeeBoxJump,
    HighPullBurpee,
    ManMakers,
    OneArmBurpee,
    SquatPlankPushUp,
    SquatThrusts,
    StandingTRotationBalance,
    WeightedBurpee,
    WeightedBurpeeBoxJump,
    WeightedSquatPlankPushUp,
    WeightedSquatThrusts,
    WeightedStandingTRotationBalance,
    UnknownValue(u64),
}

impl From<FieldContent> for TotalBodyExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => TotalBodyExerciseName::Burpee,
                1 => TotalBodyExerciseName::WeightedBurpee,
                2 => TotalBodyExerciseName::BurpeeBoxJump,
                3 => TotalBodyExerciseName::WeightedBurpeeBoxJump,
                4 => TotalBodyExerciseName::HighPullBurpee,
                5 => TotalBodyExerciseName::ManMakers,
                6 => TotalBodyExerciseName::OneArmBurpee,
                7 => TotalBodyExerciseName::SquatThrusts,
                8 => TotalBodyExerciseName::WeightedSquatThrusts,
                9 => TotalBodyExerciseName::SquatPlankPushUp,
                10 => TotalBodyExerciseName::WeightedSquatPlankPushUp,
                11 => TotalBodyExerciseName::StandingTRotationBalance,
                12 => TotalBodyExerciseName::WeightedStandingTRotationBalance,
                n => TotalBodyExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TotalBodyExerciseName to {:?}", field);
        }
    }
}
