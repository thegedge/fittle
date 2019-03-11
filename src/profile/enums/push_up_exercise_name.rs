use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PushUpExerciseName {
    AlternatingHandsMedicineBallPushUp,
    AlternatingStaggeredPushUp,
    BosuBallPushUp,
    ChestPressWithBand,
    ClappingPushUp,
    CloseGripMedicineBallPushUp,
    CloseHandsPushUp,
    DeclinePushUp,
    DiamondPushUp,
    ExplosiveCrossoverPushUp,
    ExplosivePushUp,
    FeetElevatedSideToSidePushUp,
    HandReleasePushUp,
    HandstandPushUp,
    InclinePushUp,
    IsometricExplosivePushUp,
    JudoPushUp,
    KneelingPushUp,
    MedicineBallChestPass,
    MedicineBallPushUp,
    OneArmPushUp,
    ParalletteHandstandPushUp,
    PilatesPushup,
    PushUp,
    PushUpAndRow,
    PushUpPlus,
    PushUpWithFeetOnSwissBall,
    PushUpWithOneHandOnMedicineBall,
    RingHandstandPushUp,
    RingPushUp,
    ShoulderPushUp,
    SingleArmMedicineBallPushUp,
    SpidermanPushUp,
    StackedFeetPushUp,
    StaggeredHandsPushUp,
    SuspendedPushUp,
    SwissBallPushUp,
    SwissBallPushUpPlus,
    TPushUp,
    TripleStopPushUp,
    WeightedAlternatingHandsMedicineBallPushUp,
    WeightedAlternatingStaggeredPushUp,
    WeightedBosuBallPushUp,
    WeightedClappingPushUp,
    WeightedCloseGripMedicineBallPushUp,
    WeightedCloseHandsPushUp,
    WeightedDeclinePushUp,
    WeightedDiamondPushUp,
    WeightedExplosiveCrossoverPushUp,
    WeightedExplosivePushUp,
    WeightedFeetElevatedSideToSidePushUp,
    WeightedHandReleasePushUp,
    WeightedHandstandPushUp,
    WeightedInclinePushUp,
    WeightedIsometricExplosivePushUp,
    WeightedJudoPushUp,
    WeightedKneelingPushUp,
    WeightedMedicineBallPushUp,
    WeightedOneArmPushUp,
    WeightedParalletteHandstandPushUp,
    WeightedPushUp,
    WeightedPushUpAndRow,
    WeightedPushUpPlus,
    WeightedPushUpWithFeetOnSwissBall,
    WeightedPushUpWithOneHandOnMedicineBall,
    WeightedRingHandstandPushUp,
    WeightedRingPushUp,
    WeightedShoulderPushUp,
    WeightedSingleArmMedicineBallPushUp,
    WeightedSpidermanPushUp,
    WeightedStackedFeetPushUp,
    WeightedStaggeredHandsPushUp,
    WeightedSuspendedPushUp,
    WeightedSwissBallPushUp,
    WeightedSwissBallPushUpPlus,
    WeightedTPushUp,
    WeightedTripleStopPushUp,
    WeightedWideHandsPushUp,
    WideHandsPushUp,
    UnknownValue(u64),
}

impl From<FieldContent> for PushUpExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => PushUpExerciseName::ChestPressWithBand,
                1 => PushUpExerciseName::AlternatingStaggeredPushUp,
                2 => PushUpExerciseName::WeightedAlternatingStaggeredPushUp,
                3 => PushUpExerciseName::AlternatingHandsMedicineBallPushUp,
                4 => PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp,
                5 => PushUpExerciseName::BosuBallPushUp,
                6 => PushUpExerciseName::WeightedBosuBallPushUp,
                7 => PushUpExerciseName::ClappingPushUp,
                8 => PushUpExerciseName::WeightedClappingPushUp,
                9 => PushUpExerciseName::CloseGripMedicineBallPushUp,
                10 => PushUpExerciseName::WeightedCloseGripMedicineBallPushUp,
                11 => PushUpExerciseName::CloseHandsPushUp,
                12 => PushUpExerciseName::WeightedCloseHandsPushUp,
                13 => PushUpExerciseName::DeclinePushUp,
                14 => PushUpExerciseName::WeightedDeclinePushUp,
                15 => PushUpExerciseName::DiamondPushUp,
                16 => PushUpExerciseName::WeightedDiamondPushUp,
                17 => PushUpExerciseName::ExplosiveCrossoverPushUp,
                18 => PushUpExerciseName::WeightedExplosiveCrossoverPushUp,
                19 => PushUpExerciseName::ExplosivePushUp,
                20 => PushUpExerciseName::WeightedExplosivePushUp,
                21 => PushUpExerciseName::FeetElevatedSideToSidePushUp,
                22 => PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp,
                23 => PushUpExerciseName::HandReleasePushUp,
                24 => PushUpExerciseName::WeightedHandReleasePushUp,
                25 => PushUpExerciseName::HandstandPushUp,
                26 => PushUpExerciseName::WeightedHandstandPushUp,
                27 => PushUpExerciseName::InclinePushUp,
                28 => PushUpExerciseName::WeightedInclinePushUp,
                29 => PushUpExerciseName::IsometricExplosivePushUp,
                30 => PushUpExerciseName::WeightedIsometricExplosivePushUp,
                31 => PushUpExerciseName::JudoPushUp,
                32 => PushUpExerciseName::WeightedJudoPushUp,
                33 => PushUpExerciseName::KneelingPushUp,
                34 => PushUpExerciseName::WeightedKneelingPushUp,
                35 => PushUpExerciseName::MedicineBallChestPass,
                36 => PushUpExerciseName::MedicineBallPushUp,
                37 => PushUpExerciseName::WeightedMedicineBallPushUp,
                38 => PushUpExerciseName::OneArmPushUp,
                39 => PushUpExerciseName::WeightedOneArmPushUp,
                40 => PushUpExerciseName::WeightedPushUp,
                41 => PushUpExerciseName::PushUpAndRow,
                42 => PushUpExerciseName::WeightedPushUpAndRow,
                43 => PushUpExerciseName::PushUpPlus,
                44 => PushUpExerciseName::WeightedPushUpPlus,
                45 => PushUpExerciseName::PushUpWithFeetOnSwissBall,
                46 => PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall,
                47 => PushUpExerciseName::PushUpWithOneHandOnMedicineBall,
                48 => PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall,
                49 => PushUpExerciseName::ShoulderPushUp,
                50 => PushUpExerciseName::WeightedShoulderPushUp,
                51 => PushUpExerciseName::SingleArmMedicineBallPushUp,
                52 => PushUpExerciseName::WeightedSingleArmMedicineBallPushUp,
                53 => PushUpExerciseName::SpidermanPushUp,
                54 => PushUpExerciseName::WeightedSpidermanPushUp,
                55 => PushUpExerciseName::StackedFeetPushUp,
                56 => PushUpExerciseName::WeightedStackedFeetPushUp,
                57 => PushUpExerciseName::StaggeredHandsPushUp,
                58 => PushUpExerciseName::WeightedStaggeredHandsPushUp,
                59 => PushUpExerciseName::SuspendedPushUp,
                60 => PushUpExerciseName::WeightedSuspendedPushUp,
                61 => PushUpExerciseName::SwissBallPushUp,
                62 => PushUpExerciseName::WeightedSwissBallPushUp,
                63 => PushUpExerciseName::SwissBallPushUpPlus,
                64 => PushUpExerciseName::WeightedSwissBallPushUpPlus,
                65 => PushUpExerciseName::TPushUp,
                66 => PushUpExerciseName::WeightedTPushUp,
                67 => PushUpExerciseName::TripleStopPushUp,
                68 => PushUpExerciseName::WeightedTripleStopPushUp,
                69 => PushUpExerciseName::WideHandsPushUp,
                70 => PushUpExerciseName::WeightedWideHandsPushUp,
                71 => PushUpExerciseName::ParalletteHandstandPushUp,
                72 => PushUpExerciseName::WeightedParalletteHandstandPushUp,
                73 => PushUpExerciseName::RingHandstandPushUp,
                74 => PushUpExerciseName::WeightedRingHandstandPushUp,
                75 => PushUpExerciseName::RingPushUp,
                76 => PushUpExerciseName::WeightedRingPushUp,
                77 => PushUpExerciseName::PushUp,
                78 => PushUpExerciseName::PilatesPushup,
                n => PushUpExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert PushUpExerciseName to {:?}", field);
        }
    }
}