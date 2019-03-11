use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShoulderStabilityExerciseName {
    BandExternalRotation,
    BandInternalRotation,
    BentArmLateralRaiseAndExternalRotation,
    CableExternalRotation,
    DumbbellFacePullWithExternalRotation,
    FloorIRaise,
    FloorTRaise,
    FloorYRaise,
    InclineIRaise,
    InclineLRaise,
    InclineTRaise,
    InclineWRaise,
    InclineYRaise,
    LyingExternalRotation,
    NinetyDegreeCableExternalRotation,
    SeatedDumbbellExternalRotation,
    StandingLRaise,
    SwissBallIRaise,
    SwissBallTRaise,
    SwissBallWRaise,
    SwissBallYRaise,
    WeightedFloorIRaise,
    WeightedFloorTRaise,
    WeightedFloorYRaise,
    WeightedInclineIRaise,
    WeightedInclineLRaise,
    WeightedInclineTRaise,
    WeightedInclineWRaise,
    WeightedInclineYRaise,
    WeightedSwissBallIRaise,
    WeightedSwissBallTRaise,
    WeightedSwissBallWRaise,
    WeightedSwissBallYRaise,
    UnknownValue(u64),
}

impl From<FieldContent> for ShoulderStabilityExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => ShoulderStabilityExerciseName::NinetyDegreeCableExternalRotation,
                1 => ShoulderStabilityExerciseName::BandExternalRotation,
                2 => ShoulderStabilityExerciseName::BandInternalRotation,
                3 => ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation,
                4 => ShoulderStabilityExerciseName::CableExternalRotation,
                5 => ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation,
                6 => ShoulderStabilityExerciseName::FloorIRaise,
                7 => ShoulderStabilityExerciseName::WeightedFloorIRaise,
                8 => ShoulderStabilityExerciseName::FloorTRaise,
                9 => ShoulderStabilityExerciseName::WeightedFloorTRaise,
                10 => ShoulderStabilityExerciseName::FloorYRaise,
                11 => ShoulderStabilityExerciseName::WeightedFloorYRaise,
                12 => ShoulderStabilityExerciseName::InclineIRaise,
                13 => ShoulderStabilityExerciseName::WeightedInclineIRaise,
                14 => ShoulderStabilityExerciseName::InclineLRaise,
                15 => ShoulderStabilityExerciseName::WeightedInclineLRaise,
                16 => ShoulderStabilityExerciseName::InclineTRaise,
                17 => ShoulderStabilityExerciseName::WeightedInclineTRaise,
                18 => ShoulderStabilityExerciseName::InclineWRaise,
                19 => ShoulderStabilityExerciseName::WeightedInclineWRaise,
                20 => ShoulderStabilityExerciseName::InclineYRaise,
                21 => ShoulderStabilityExerciseName::WeightedInclineYRaise,
                22 => ShoulderStabilityExerciseName::LyingExternalRotation,
                23 => ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation,
                24 => ShoulderStabilityExerciseName::StandingLRaise,
                25 => ShoulderStabilityExerciseName::SwissBallIRaise,
                26 => ShoulderStabilityExerciseName::WeightedSwissBallIRaise,
                27 => ShoulderStabilityExerciseName::SwissBallTRaise,
                28 => ShoulderStabilityExerciseName::WeightedSwissBallTRaise,
                29 => ShoulderStabilityExerciseName::SwissBallWRaise,
                30 => ShoulderStabilityExerciseName::WeightedSwissBallWRaise,
                31 => ShoulderStabilityExerciseName::SwissBallYRaise,
                32 => ShoulderStabilityExerciseName::WeightedSwissBallYRaise,
                n => ShoulderStabilityExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ShoulderStabilityExerciseName to {:?}", field);
        }
    }
}
