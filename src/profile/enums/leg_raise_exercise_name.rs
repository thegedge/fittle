use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegRaiseExerciseName {
    HangingKneeRaise,
    HangingLegRaise,
    HangingSingleLegRaise,
    KettlebellLegRaises,
    LateralStepover,
    LegLoweringDrill,
    LyingStraightLegRaise,
    MedicineBallLegDrops,
    QuadrupedLegRaise,
    ReverseLegRaise,
    ReverseLegRaiseOnSwissBall,
    SingleLegLoweringDrill,
    WeightedHangingKneeRaise,
    WeightedHangingLegRaise,
    WeightedHangingSingleLegRaise,
    WeightedLateralStepover,
    WeightedLegLoweringDrill,
    WeightedLyingStraightLegRaise,
    WeightedQuadrupedLegRaise,
    WeightedReverseLegRaise,
    WeightedReverseLegRaiseOnSwissBall,
    WeightedSingleLegLoweringDrill,
    UnknownValue(u64),
}

impl From<FieldContent> for LegRaiseExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => LegRaiseExerciseName::HangingKneeRaise,
                1 => LegRaiseExerciseName::HangingLegRaise,
                2 => LegRaiseExerciseName::WeightedHangingLegRaise,
                3 => LegRaiseExerciseName::HangingSingleLegRaise,
                4 => LegRaiseExerciseName::WeightedHangingSingleLegRaise,
                5 => LegRaiseExerciseName::KettlebellLegRaises,
                6 => LegRaiseExerciseName::LegLoweringDrill,
                7 => LegRaiseExerciseName::WeightedLegLoweringDrill,
                8 => LegRaiseExerciseName::LyingStraightLegRaise,
                9 => LegRaiseExerciseName::WeightedLyingStraightLegRaise,
                10 => LegRaiseExerciseName::MedicineBallLegDrops,
                11 => LegRaiseExerciseName::QuadrupedLegRaise,
                12 => LegRaiseExerciseName::WeightedQuadrupedLegRaise,
                13 => LegRaiseExerciseName::ReverseLegRaise,
                14 => LegRaiseExerciseName::WeightedReverseLegRaise,
                15 => LegRaiseExerciseName::ReverseLegRaiseOnSwissBall,
                16 => LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall,
                17 => LegRaiseExerciseName::SingleLegLoweringDrill,
                18 => LegRaiseExerciseName::WeightedSingleLegLoweringDrill,
                19 => LegRaiseExerciseName::WeightedHangingKneeRaise,
                20 => LegRaiseExerciseName::LateralStepover,
                21 => LegRaiseExerciseName::WeightedLateralStepover,
                n => LegRaiseExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LegRaiseExerciseName to {:?}", field);
        }
    }
}
