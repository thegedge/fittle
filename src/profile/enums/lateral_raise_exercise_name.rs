use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LateralRaiseExerciseName {
    AlternatingLateralRaiseWithStaticHold,
    ArmCircles,
    BarMuscleUp,
    BentOverLateralRaise,
    CableDiagonalRaise,
    CableFrontRaise,
    CalorieRow,
    ComboShoulderRaise,
    DumbbellDiagonalRaise,
    DumbbellVRaise,
    FortyFiveDegreeCableExternalRotation,
    FrontRaise,
    LeaningDumbbellLateralRaise,
    LyingDumbbellRaise,
    MuscleUp,
    OneArmCableLateralRaise,
    OverhandGripRearLateralRaise,
    PlateRaises,
    RingDip,
    RingMuscleUp,
    RopeClimb,
    Scaption,
    SeatedLateralRaise,
    SeatedRearLateralRaise,
    ShavingTheHead,
    SideLyingLateralRaise,
    StandingLift,
    SuspendedRow,
    UnderhandGripRearLateralRaise,
    WallSlide,
    WeightedRingDip,
    WeightedRingMuscleUp,
    WeightedRopeClimb,
    WeightedWallSlide,
    UnknownValue(u64),
}

impl From<FieldContent> for LateralRaiseExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => LateralRaiseExerciseName::FortyFiveDegreeCableExternalRotation,
                1 => LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold,
                2 => LateralRaiseExerciseName::BarMuscleUp,
                3 => LateralRaiseExerciseName::BentOverLateralRaise,
                4 => LateralRaiseExerciseName::CableDiagonalRaise,
                5 => LateralRaiseExerciseName::CableFrontRaise,
                6 => LateralRaiseExerciseName::CalorieRow,
                7 => LateralRaiseExerciseName::ComboShoulderRaise,
                8 => LateralRaiseExerciseName::DumbbellDiagonalRaise,
                9 => LateralRaiseExerciseName::DumbbellVRaise,
                10 => LateralRaiseExerciseName::FrontRaise,
                11 => LateralRaiseExerciseName::LeaningDumbbellLateralRaise,
                12 => LateralRaiseExerciseName::LyingDumbbellRaise,
                13 => LateralRaiseExerciseName::MuscleUp,
                14 => LateralRaiseExerciseName::OneArmCableLateralRaise,
                15 => LateralRaiseExerciseName::OverhandGripRearLateralRaise,
                16 => LateralRaiseExerciseName::PlateRaises,
                17 => LateralRaiseExerciseName::RingDip,
                18 => LateralRaiseExerciseName::WeightedRingDip,
                19 => LateralRaiseExerciseName::RingMuscleUp,
                20 => LateralRaiseExerciseName::WeightedRingMuscleUp,
                21 => LateralRaiseExerciseName::RopeClimb,
                22 => LateralRaiseExerciseName::WeightedRopeClimb,
                23 => LateralRaiseExerciseName::Scaption,
                24 => LateralRaiseExerciseName::SeatedLateralRaise,
                25 => LateralRaiseExerciseName::SeatedRearLateralRaise,
                26 => LateralRaiseExerciseName::SideLyingLateralRaise,
                27 => LateralRaiseExerciseName::StandingLift,
                28 => LateralRaiseExerciseName::SuspendedRow,
                29 => LateralRaiseExerciseName::UnderhandGripRearLateralRaise,
                30 => LateralRaiseExerciseName::WallSlide,
                31 => LateralRaiseExerciseName::WeightedWallSlide,
                32 => LateralRaiseExerciseName::ArmCircles,
                33 => LateralRaiseExerciseName::ShavingTheHead,
                n => LateralRaiseExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LateralRaiseExerciseName to {:?}", field);
        }
    }
}
