use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlyoExerciseName {
    AlternatingJumpLunge,
    BarbellJumpSquat,
    BodyWeightJumpSquat,
    CrossKneeStrike,
    DepthJump,
    DumbbellJumpSquat,
    DumbbellSplitJump,
    FrontKneeStrike,
    HighBoxJump,
    IsometricExplosiveBodyWeightJumpSquat,
    LateralLeapAndHop,
    LateralPlyoSquats,
    LateralSlide,
    MedicineBallOverheadThrows,
    MedicineBallSideThrow,
    MedicineBallSlam,
    SideToSideMedicineBallThrows,
    SideToSideShuffleJump,
    SquatJumpOntoBox,
    SquatJumpsInAndOut,
    WeightedAlternatingJumpLunge,
    WeightedCrossKneeStrike,
    WeightedDepthJump,
    WeightedFrontKneeStrike,
    WeightedHighBoxJump,
    WeightedIsometricExplosiveJumpSquat,
    WeightedJumpSquat,
    WeightedLateralLeapAndHop,
    WeightedLateralPlyoSquats,
    WeightedLateralSlide,
    WeightedSideToSideShuffleJump,
    WeightedSquatJumpOntoBox,
    WeightedSquatJumpsInAndOut,
    UnknownValue(u64),
}

impl From<FieldContent> for PlyoExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => PlyoExerciseName::AlternatingJumpLunge,
                1 => PlyoExerciseName::WeightedAlternatingJumpLunge,
                2 => PlyoExerciseName::BarbellJumpSquat,
                3 => PlyoExerciseName::BodyWeightJumpSquat,
                4 => PlyoExerciseName::WeightedJumpSquat,
                5 => PlyoExerciseName::CrossKneeStrike,
                6 => PlyoExerciseName::WeightedCrossKneeStrike,
                7 => PlyoExerciseName::DepthJump,
                8 => PlyoExerciseName::WeightedDepthJump,
                9 => PlyoExerciseName::DumbbellJumpSquat,
                10 => PlyoExerciseName::DumbbellSplitJump,
                11 => PlyoExerciseName::FrontKneeStrike,
                12 => PlyoExerciseName::WeightedFrontKneeStrike,
                13 => PlyoExerciseName::HighBoxJump,
                14 => PlyoExerciseName::WeightedHighBoxJump,
                15 => PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat,
                16 => PlyoExerciseName::WeightedIsometricExplosiveJumpSquat,
                17 => PlyoExerciseName::LateralLeapAndHop,
                18 => PlyoExerciseName::WeightedLateralLeapAndHop,
                19 => PlyoExerciseName::LateralPlyoSquats,
                20 => PlyoExerciseName::WeightedLateralPlyoSquats,
                21 => PlyoExerciseName::LateralSlide,
                22 => PlyoExerciseName::WeightedLateralSlide,
                23 => PlyoExerciseName::MedicineBallOverheadThrows,
                24 => PlyoExerciseName::MedicineBallSideThrow,
                25 => PlyoExerciseName::MedicineBallSlam,
                26 => PlyoExerciseName::SideToSideMedicineBallThrows,
                27 => PlyoExerciseName::SideToSideShuffleJump,
                28 => PlyoExerciseName::WeightedSideToSideShuffleJump,
                29 => PlyoExerciseName::SquatJumpOntoBox,
                30 => PlyoExerciseName::WeightedSquatJumpOntoBox,
                31 => PlyoExerciseName::SquatJumpsInAndOut,
                32 => PlyoExerciseName::WeightedSquatJumpsInAndOut,
                n => PlyoExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert PlyoExerciseName to {:?}", field);
        }
    }
}
