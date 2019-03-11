use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmUpExerciseName {
    AnkleCircles,
    AnkleDorsiflexionWithBand,
    AnkleInternalRotation,
    ArmCircles,
    BentOverReachToSky,
    CatCamel,
    ElbowToFootLunge,
    ForwardAndBackwardLegSwings,
    Groiners,
    InvertedHamstringStretch,
    LateralDuckUnder,
    NeckRotations,
    NeckTilts,
    OppositeArmAndLegBalance,
    QuadrupedRocking,
    ReachRollAndLift,
    Scorpion,
    ShoulderCircles,
    SideToSideLegSwings,
    SleeperStretch,
    SlideOut,
    SwissBallHipCrossover,
    SwissBallReachRollAndLift,
    SwissBallWindshieldWipers,
    ThoracicRotation,
    WalkingHighKicks,
    WalkingHighKnees,
    WalkingKneeHugs,
    WalkingLegCradles,
    Walkout,
    WalkoutFromPushUpPosition,
    UnknownValue(u64),
}

impl From<FieldContent> for WarmUpExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => WarmUpExerciseName::QuadrupedRocking,
                1 => WarmUpExerciseName::NeckTilts,
                2 => WarmUpExerciseName::AnkleCircles,
                3 => WarmUpExerciseName::AnkleDorsiflexionWithBand,
                4 => WarmUpExerciseName::AnkleInternalRotation,
                5 => WarmUpExerciseName::ArmCircles,
                6 => WarmUpExerciseName::BentOverReachToSky,
                7 => WarmUpExerciseName::CatCamel,
                8 => WarmUpExerciseName::ElbowToFootLunge,
                9 => WarmUpExerciseName::ForwardAndBackwardLegSwings,
                10 => WarmUpExerciseName::Groiners,
                11 => WarmUpExerciseName::InvertedHamstringStretch,
                12 => WarmUpExerciseName::LateralDuckUnder,
                13 => WarmUpExerciseName::NeckRotations,
                14 => WarmUpExerciseName::OppositeArmAndLegBalance,
                15 => WarmUpExerciseName::ReachRollAndLift,
                16 => WarmUpExerciseName::Scorpion,
                17 => WarmUpExerciseName::ShoulderCircles,
                18 => WarmUpExerciseName::SideToSideLegSwings,
                19 => WarmUpExerciseName::SleeperStretch,
                20 => WarmUpExerciseName::SlideOut,
                21 => WarmUpExerciseName::SwissBallHipCrossover,
                22 => WarmUpExerciseName::SwissBallReachRollAndLift,
                23 => WarmUpExerciseName::SwissBallWindshieldWipers,
                24 => WarmUpExerciseName::ThoracicRotation,
                25 => WarmUpExerciseName::WalkingHighKicks,
                26 => WarmUpExerciseName::WalkingHighKnees,
                27 => WarmUpExerciseName::WalkingKneeHugs,
                28 => WarmUpExerciseName::WalkingLegCradles,
                29 => WarmUpExerciseName::Walkout,
                30 => WarmUpExerciseName::WalkoutFromPushUpPosition,
                n => WarmUpExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WarmUpExerciseName to {:?}", field);
        }
    }
}
