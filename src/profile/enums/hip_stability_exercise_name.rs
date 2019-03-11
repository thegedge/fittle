use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HipStabilityExerciseName {
    BandSideLyingLegRaise,
    DeadBug,
    ExternalHipRaise,
    FireHydrantKicks,
    HipCircles,
    InnerThighLift,
    LateralWalksWithBandAtAnkles,
    PretzelSideKick,
    ProneHipInternalRotation,
    Quadruped,
    QuadrupedHipExtension,
    QuadrupedWithLegLift,
    SideLyingLegRaise,
    SlidingHipAdduction,
    StandingAdduction,
    StandingCableHipAbduction,
    StandingHipAbduction,
    StandingRearLegRaise,
    SupineHipInternalRotation,
    WeightedDeadBug,
    WeightedExternalHipRaise,
    WeightedFireHydrantKicks,
    WeightedHipCircles,
    WeightedInnerThighLift,
    WeightedPretzelSideKick,
    WeightedProneHipInternalRotation,
    WeightedQuadrupedHipExtension,
    WeightedQuadrupedWithLegLift,
    WeightedSideLyingLegRaise,
    WeightedSlidingHipAdduction,
    WeightedStandingAdduction,
    WeightedStandingHipAbduction,
    WeightedStandingRearLegRaise,
    WeightedSupineHipInternalRotation,
    UnknownValue(u64),
}

impl From<FieldContent> for HipStabilityExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => HipStabilityExerciseName::BandSideLyingLegRaise,
                1 => HipStabilityExerciseName::DeadBug,
                2 => HipStabilityExerciseName::WeightedDeadBug,
                3 => HipStabilityExerciseName::ExternalHipRaise,
                4 => HipStabilityExerciseName::WeightedExternalHipRaise,
                5 => HipStabilityExerciseName::FireHydrantKicks,
                6 => HipStabilityExerciseName::WeightedFireHydrantKicks,
                7 => HipStabilityExerciseName::HipCircles,
                8 => HipStabilityExerciseName::WeightedHipCircles,
                9 => HipStabilityExerciseName::InnerThighLift,
                10 => HipStabilityExerciseName::WeightedInnerThighLift,
                11 => HipStabilityExerciseName::LateralWalksWithBandAtAnkles,
                12 => HipStabilityExerciseName::PretzelSideKick,
                13 => HipStabilityExerciseName::WeightedPretzelSideKick,
                14 => HipStabilityExerciseName::ProneHipInternalRotation,
                15 => HipStabilityExerciseName::WeightedProneHipInternalRotation,
                16 => HipStabilityExerciseName::Quadruped,
                17 => HipStabilityExerciseName::QuadrupedHipExtension,
                18 => HipStabilityExerciseName::WeightedQuadrupedHipExtension,
                19 => HipStabilityExerciseName::QuadrupedWithLegLift,
                20 => HipStabilityExerciseName::WeightedQuadrupedWithLegLift,
                21 => HipStabilityExerciseName::SideLyingLegRaise,
                22 => HipStabilityExerciseName::WeightedSideLyingLegRaise,
                23 => HipStabilityExerciseName::SlidingHipAdduction,
                24 => HipStabilityExerciseName::WeightedSlidingHipAdduction,
                25 => HipStabilityExerciseName::StandingAdduction,
                26 => HipStabilityExerciseName::WeightedStandingAdduction,
                27 => HipStabilityExerciseName::StandingCableHipAbduction,
                28 => HipStabilityExerciseName::StandingHipAbduction,
                29 => HipStabilityExerciseName::WeightedStandingHipAbduction,
                30 => HipStabilityExerciseName::StandingRearLegRaise,
                31 => HipStabilityExerciseName::WeightedStandingRearLegRaise,
                32 => HipStabilityExerciseName::SupineHipInternalRotation,
                33 => HipStabilityExerciseName::WeightedSupineHipInternalRotation,
                n => HipStabilityExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert HipStabilityExerciseName to {:?}", field);
        }
    }
}
