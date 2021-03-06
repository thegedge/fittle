use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlankExerciseName {
    BearCrawl,
    BridgeOneLegBridge,
    BridgeWithGluteLowerLift,
    CrossBodyMountainClimber,
    ElbowPlankPikeJacks,
    ElevatedFeetPlank,
    ElevatorAbs,
    ExtendedPlank,
    FortyFiveDegreePlank,
    FullPlankPasseTwist,
    InchingElbowPlank,
    InchwormToSidePlank,
    KneelingPlank,
    KneelingSidePlankWithLegLift,
    LateralRoll,
    LyingReversePlank,
    MedicineBallMountainClimber,
    ModifiedMountainClimberAndExtension,
    MountainClimber,
    MountainClimberOnSlidingDiscs,
    MountainClimberWithFeetOnBosuBall,
    MountainClimberWithHandsOnBench,
    MountainClimberWithHandsOnSwissBall,
    NinetyDegreePlank,
    Plank,
    PlankJacksWithFeetOnSlidingDiscs,
    PlankKneeTwist,
    PlankPikeJumps,
    PlankPikes,
    PlankToStandUp,
    PlankWithArmRaise,
    PlankWithArmVariations,
    PlankWithFeetOnSwissBall,
    PlankWithKneeToElbow,
    PlankWithLegLift,
    PlankWithObliqueCrunch,
    PlyometricSidePlank,
    ReversePlankWithLegPull,
    RollingSidePlank,
    SideKickPlank,
    SidePlank,
    SidePlankAndRow,
    SidePlankLift,
    SidePlankToPlankWithReachUnder,
    SidePlankWithElbowOnBosuBall,
    SidePlankWithFeetOnBench,
    SidePlankWithKneeCircle,
    SidePlankWithKneeTuck,
    SidePlankWithLegLift,
    SidePlankWithReachUnder,
    SingleLegElevatedFeetPlank,
    SingleLegFlexAndExtend,
    SingleLegSidePlank,
    SpidermanPlank,
    StraightArmPlank,
    StraightArmPlankWithShoulderTouch,
    SwissBallPlank,
    SwissBallPlankLegLift,
    SwissBallPlankLegLiftAndHold,
    SwissBallPlankWithFeetOnBench,
    SwissBallProneJackknife,
    SwissBallSidePlank,
    TStabilization,
    ThreeWayPlank,
    TowelPlankAndKneeIn,
    TurkishGetUpToSidePlank,
    TwoPointPlank,
    Weighted45DegreePlank,
    Weighted90DegreeStaticHold,
    WeightedBearCrawl,
    WeightedCrossBodyMountainClimber,
    WeightedElbowPlankPikeJacks,
    WeightedElevatedFeetPlank,
    WeightedElevatorAbs,
    WeightedExtendedPlank,
    WeightedFullPlankPasseTwist,
    WeightedInchingElbowPlank,
    WeightedInchwormToSidePlank,
    WeightedKneelingPlank,
    WeightedKneelingSidePlankWithLegLift,
    WeightedLateralRoll,
    WeightedLyingReversePlank,
    WeightedMedicineBallMountainClimber,
    WeightedModifiedMountainClimberAndExtension,
    WeightedMountainClimber,
    WeightedMountainClimberOnSlidingDiscs,
    WeightedMountainClimberWithFeetOnBosuBall,
    WeightedMountainClimberWithHandsOnBench,
    WeightedMountainClimberWithHandsOnSwissBall,
    WeightedPlank,
    WeightedPlankJacksWithFeetOnSlidingDiscs,
    WeightedPlankKneeTwist,
    WeightedPlankPikeJumps,
    WeightedPlankPikes,
    WeightedPlankToStandUp,
    WeightedPlankWithArmRaise,
    WeightedPlankWithKneeToElbow,
    WeightedPlankWithObliqueCrunch,
    WeightedPlyometricSidePlank,
    WeightedRollingSidePlank,
    WeightedSideKickPlank,
    WeightedSidePlank,
    WeightedSidePlankAndRow,
    WeightedSidePlankLift,
    WeightedSidePlankWithElbowOnBosuBall,
    WeightedSidePlankWithFeetOnBench,
    WeightedSidePlankWithKneeCircle,
    WeightedSidePlankWithKneeTuck,
    WeightedSidePlankWithLegLift,
    WeightedSidePlankWithReachUnder,
    WeightedSingleLegElevatedFeetPlank,
    WeightedSingleLegFlexAndExtend,
    WeightedSingleLegSidePlank,
    WeightedSpidermanPlank,
    WeightedStraightArmPlank,
    WeightedStraightArmPlankWithShoulderTouch,
    WeightedSwissBallPlank,
    WeightedSwissBallPlankLegLift,
    WeightedSwissBallPlankLegLiftAndHold,
    WeightedSwissBallPlankWithFeetOnBench,
    WeightedSwissBallProneJackknife,
    WeightedSwissBallSidePlank,
    WeightedTStabilization,
    WeightedThreeWayPlank,
    WeightedTowelPlankAndKneeIn,
    WeightedTurkishGetUpToSidePlank,
    WeightedTwoPointPlank,
    WeightedWideStancePlankWithDiagonalArmLift,
    WeightedWideStancePlankWithDiagonalLegLift,
    WeightedWideStancePlankWithLegLift,
    WeightedWideStancePlankWithOppositeArmAndLegLift,
    WideStancePlankWithDiagonalArmLift,
    WideStancePlankWithDiagonalLegLift,
    WideStancePlankWithLegLift,
    WideStancePlankWithOppositeArmAndLegLift,
    UnknownValue(u64),
}

impl From<FieldContent> for PlankExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => PlankExerciseName::FortyFiveDegreePlank,
                1 => PlankExerciseName::Weighted45DegreePlank,
                2 => PlankExerciseName::NinetyDegreePlank,
                3 => PlankExerciseName::Weighted90DegreeStaticHold,
                4 => PlankExerciseName::BearCrawl,
                5 => PlankExerciseName::WeightedBearCrawl,
                6 => PlankExerciseName::CrossBodyMountainClimber,
                7 => PlankExerciseName::WeightedCrossBodyMountainClimber,
                8 => PlankExerciseName::ElbowPlankPikeJacks,
                9 => PlankExerciseName::WeightedElbowPlankPikeJacks,
                10 => PlankExerciseName::ElevatedFeetPlank,
                11 => PlankExerciseName::WeightedElevatedFeetPlank,
                12 => PlankExerciseName::ElevatorAbs,
                13 => PlankExerciseName::WeightedElevatorAbs,
                14 => PlankExerciseName::ExtendedPlank,
                15 => PlankExerciseName::WeightedExtendedPlank,
                16 => PlankExerciseName::FullPlankPasseTwist,
                17 => PlankExerciseName::WeightedFullPlankPasseTwist,
                18 => PlankExerciseName::InchingElbowPlank,
                19 => PlankExerciseName::WeightedInchingElbowPlank,
                20 => PlankExerciseName::InchwormToSidePlank,
                21 => PlankExerciseName::WeightedInchwormToSidePlank,
                22 => PlankExerciseName::KneelingPlank,
                23 => PlankExerciseName::WeightedKneelingPlank,
                24 => PlankExerciseName::KneelingSidePlankWithLegLift,
                25 => PlankExerciseName::WeightedKneelingSidePlankWithLegLift,
                26 => PlankExerciseName::LateralRoll,
                27 => PlankExerciseName::WeightedLateralRoll,
                28 => PlankExerciseName::LyingReversePlank,
                29 => PlankExerciseName::WeightedLyingReversePlank,
                30 => PlankExerciseName::MedicineBallMountainClimber,
                31 => PlankExerciseName::WeightedMedicineBallMountainClimber,
                32 => PlankExerciseName::ModifiedMountainClimberAndExtension,
                33 => PlankExerciseName::WeightedModifiedMountainClimberAndExtension,
                34 => PlankExerciseName::MountainClimber,
                35 => PlankExerciseName::WeightedMountainClimber,
                36 => PlankExerciseName::MountainClimberOnSlidingDiscs,
                37 => PlankExerciseName::WeightedMountainClimberOnSlidingDiscs,
                38 => PlankExerciseName::MountainClimberWithFeetOnBosuBall,
                39 => PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall,
                40 => PlankExerciseName::MountainClimberWithHandsOnBench,
                41 => PlankExerciseName::MountainClimberWithHandsOnSwissBall,
                42 => PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall,
                43 => PlankExerciseName::Plank,
                44 => PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs,
                45 => PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs,
                46 => PlankExerciseName::PlankKneeTwist,
                47 => PlankExerciseName::WeightedPlankKneeTwist,
                48 => PlankExerciseName::PlankPikeJumps,
                49 => PlankExerciseName::WeightedPlankPikeJumps,
                50 => PlankExerciseName::PlankPikes,
                51 => PlankExerciseName::WeightedPlankPikes,
                52 => PlankExerciseName::PlankToStandUp,
                53 => PlankExerciseName::WeightedPlankToStandUp,
                54 => PlankExerciseName::PlankWithArmRaise,
                55 => PlankExerciseName::WeightedPlankWithArmRaise,
                56 => PlankExerciseName::PlankWithKneeToElbow,
                57 => PlankExerciseName::WeightedPlankWithKneeToElbow,
                58 => PlankExerciseName::PlankWithObliqueCrunch,
                59 => PlankExerciseName::WeightedPlankWithObliqueCrunch,
                60 => PlankExerciseName::PlyometricSidePlank,
                61 => PlankExerciseName::WeightedPlyometricSidePlank,
                62 => PlankExerciseName::RollingSidePlank,
                63 => PlankExerciseName::WeightedRollingSidePlank,
                64 => PlankExerciseName::SideKickPlank,
                65 => PlankExerciseName::WeightedSideKickPlank,
                66 => PlankExerciseName::SidePlank,
                67 => PlankExerciseName::WeightedSidePlank,
                68 => PlankExerciseName::SidePlankAndRow,
                69 => PlankExerciseName::WeightedSidePlankAndRow,
                70 => PlankExerciseName::SidePlankLift,
                71 => PlankExerciseName::WeightedSidePlankLift,
                72 => PlankExerciseName::SidePlankWithElbowOnBosuBall,
                73 => PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall,
                74 => PlankExerciseName::SidePlankWithFeetOnBench,
                75 => PlankExerciseName::WeightedSidePlankWithFeetOnBench,
                76 => PlankExerciseName::SidePlankWithKneeCircle,
                77 => PlankExerciseName::WeightedSidePlankWithKneeCircle,
                78 => PlankExerciseName::SidePlankWithKneeTuck,
                79 => PlankExerciseName::WeightedSidePlankWithKneeTuck,
                80 => PlankExerciseName::SidePlankWithLegLift,
                81 => PlankExerciseName::WeightedSidePlankWithLegLift,
                82 => PlankExerciseName::SidePlankWithReachUnder,
                83 => PlankExerciseName::WeightedSidePlankWithReachUnder,
                84 => PlankExerciseName::SingleLegElevatedFeetPlank,
                85 => PlankExerciseName::WeightedSingleLegElevatedFeetPlank,
                86 => PlankExerciseName::SingleLegFlexAndExtend,
                87 => PlankExerciseName::WeightedSingleLegFlexAndExtend,
                88 => PlankExerciseName::SingleLegSidePlank,
                89 => PlankExerciseName::WeightedSingleLegSidePlank,
                90 => PlankExerciseName::SpidermanPlank,
                91 => PlankExerciseName::WeightedSpidermanPlank,
                92 => PlankExerciseName::StraightArmPlank,
                93 => PlankExerciseName::WeightedStraightArmPlank,
                94 => PlankExerciseName::StraightArmPlankWithShoulderTouch,
                95 => PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch,
                96 => PlankExerciseName::SwissBallPlank,
                97 => PlankExerciseName::WeightedSwissBallPlank,
                98 => PlankExerciseName::SwissBallPlankLegLift,
                99 => PlankExerciseName::WeightedSwissBallPlankLegLift,
                100 => PlankExerciseName::SwissBallPlankLegLiftAndHold,
                101 => PlankExerciseName::SwissBallPlankWithFeetOnBench,
                102 => PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench,
                103 => PlankExerciseName::SwissBallProneJackknife,
                104 => PlankExerciseName::WeightedSwissBallProneJackknife,
                105 => PlankExerciseName::SwissBallSidePlank,
                106 => PlankExerciseName::WeightedSwissBallSidePlank,
                107 => PlankExerciseName::ThreeWayPlank,
                108 => PlankExerciseName::WeightedThreeWayPlank,
                109 => PlankExerciseName::TowelPlankAndKneeIn,
                110 => PlankExerciseName::WeightedTowelPlankAndKneeIn,
                111 => PlankExerciseName::TStabilization,
                112 => PlankExerciseName::WeightedTStabilization,
                113 => PlankExerciseName::TurkishGetUpToSidePlank,
                114 => PlankExerciseName::WeightedTurkishGetUpToSidePlank,
                115 => PlankExerciseName::TwoPointPlank,
                116 => PlankExerciseName::WeightedTwoPointPlank,
                117 => PlankExerciseName::WeightedPlank,
                118 => PlankExerciseName::WideStancePlankWithDiagonalArmLift,
                119 => PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift,
                120 => PlankExerciseName::WideStancePlankWithDiagonalLegLift,
                121 => PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift,
                122 => PlankExerciseName::WideStancePlankWithLegLift,
                123 => PlankExerciseName::WeightedWideStancePlankWithLegLift,
                124 => PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift,
                125 => PlankExerciseName::WeightedMountainClimberWithHandsOnBench,
                126 => PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold,
                127 => PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift,
                128 => PlankExerciseName::PlankWithFeetOnSwissBall,
                129 => PlankExerciseName::SidePlankToPlankWithReachUnder,
                130 => PlankExerciseName::BridgeWithGluteLowerLift,
                131 => PlankExerciseName::BridgeOneLegBridge,
                132 => PlankExerciseName::PlankWithArmVariations,
                133 => PlankExerciseName::PlankWithLegLift,
                134 => PlankExerciseName::ReversePlankWithLegPull,
                n => PlankExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert PlankExerciseName to {:?}", field);
        }
    }
}
