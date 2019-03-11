use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChopExerciseName {
    CablePullThrough,
    CableRotationalLift,
    CableWoodchop,
    CrossChopToKnee,
    DumbbellChop,
    HalfKneelingRotation,
    HalfKneelingRotationalChop,
    HalfKneelingRotationalReverseChop,
    HalfKneelingStabilityChop,
    HalfKneelingStabilityReverseChop,
    KneelingRotationalChop,
    KneelingRotationalReverseChop,
    KneelingStabilityChop,
    KneelingWoodchopper,
    MedicineBallWoodChops,
    PowerSquatChops,
    StandingRotationalChop,
    StandingSplitRotationalChop,
    StandingSplitRotationalReverseChop,
    StandingStabilityReverseChop,
    WeightedCrossChopToKnee,
    WeightedHalfKneelingRotation,
    WeightedPowerSquatChops,
    UnknownValue(u64),
}

impl From<FieldContent> for ChopExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => ChopExerciseName::CablePullThrough,
                1 => ChopExerciseName::CableRotationalLift,
                2 => ChopExerciseName::CableWoodchop,
                3 => ChopExerciseName::CrossChopToKnee,
                4 => ChopExerciseName::WeightedCrossChopToKnee,
                5 => ChopExerciseName::DumbbellChop,
                6 => ChopExerciseName::HalfKneelingRotation,
                7 => ChopExerciseName::WeightedHalfKneelingRotation,
                8 => ChopExerciseName::HalfKneelingRotationalChop,
                9 => ChopExerciseName::HalfKneelingRotationalReverseChop,
                10 => ChopExerciseName::HalfKneelingStabilityChop,
                11 => ChopExerciseName::HalfKneelingStabilityReverseChop,
                12 => ChopExerciseName::KneelingRotationalChop,
                13 => ChopExerciseName::KneelingRotationalReverseChop,
                14 => ChopExerciseName::KneelingStabilityChop,
                15 => ChopExerciseName::KneelingWoodchopper,
                16 => ChopExerciseName::MedicineBallWoodChops,
                17 => ChopExerciseName::PowerSquatChops,
                18 => ChopExerciseName::WeightedPowerSquatChops,
                19 => ChopExerciseName::StandingRotationalChop,
                20 => ChopExerciseName::StandingSplitRotationalChop,
                21 => ChopExerciseName::StandingSplitRotationalReverseChop,
                22 => ChopExerciseName::StandingStabilityReverseChop,
                n => ChopExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ChopExerciseName to {:?}", field);
        }
    }
}
