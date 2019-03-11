use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeadliftExerciseName {
    BarbellDeadlift,
    BarbellStraightLegDeadlift,
    DumbbellDeadlift,
    DumbbellSingleLegDeadliftToRow,
    DumbbellStraightLegDeadlift,
    KettlebellFloorToShelf,
    OneArmOneLegDeadlift,
    RackPull,
    RotationalDumbbellStraightLegDeadlift,
    SingleArmDeadlift,
    SingleLegBarbellDeadlift,
    SingleLegBarbellStraightLegDeadlift,
    SingleLegDeadliftWithBarbell,
    SingleLegRdlCircuit,
    SingleLegRomanianDeadliftWithDumbbell,
    SumoDeadlift,
    SumoDeadliftHighPull,
    TrapBarDeadlift,
    WideGripBarbellDeadlift,
    UnknownValue(u64),
}

impl From<FieldContent> for DeadliftExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => DeadliftExerciseName::BarbellDeadlift,
                1 => DeadliftExerciseName::BarbellStraightLegDeadlift,
                2 => DeadliftExerciseName::DumbbellDeadlift,
                3 => DeadliftExerciseName::DumbbellSingleLegDeadliftToRow,
                4 => DeadliftExerciseName::DumbbellStraightLegDeadlift,
                5 => DeadliftExerciseName::KettlebellFloorToShelf,
                6 => DeadliftExerciseName::OneArmOneLegDeadlift,
                7 => DeadliftExerciseName::RackPull,
                8 => DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift,
                9 => DeadliftExerciseName::SingleArmDeadlift,
                10 => DeadliftExerciseName::SingleLegBarbellDeadlift,
                11 => DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift,
                12 => DeadliftExerciseName::SingleLegDeadliftWithBarbell,
                13 => DeadliftExerciseName::SingleLegRdlCircuit,
                14 => DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell,
                15 => DeadliftExerciseName::SumoDeadlift,
                16 => DeadliftExerciseName::SumoDeadliftHighPull,
                17 => DeadliftExerciseName::TrapBarDeadlift,
                18 => DeadliftExerciseName::WideGripBarbellDeadlift,
                n => DeadliftExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DeadliftExerciseName to {:?}", field);
        }
    }
}
