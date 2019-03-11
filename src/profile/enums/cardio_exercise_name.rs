use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CardioExerciseName {
    BobAndWeaveCircle,
    CardioCoreCrawl,
    DoubleUnder,
    JumpRope,
    JumpRopeCrossover,
    JumpRopeJog,
    JumpingJacks,
    SkiMoguls,
    SplitJacks,
    SquatJacks,
    TripleUnder,
    WeightedBobAndWeaveCircle,
    WeightedCardioCoreCrawl,
    WeightedDoubleUnder,
    WeightedJumpRope,
    WeightedJumpRopeCrossover,
    WeightedJumpRopeJog,
    WeightedJumpingJacks,
    WeightedSkiMoguls,
    WeightedSplitJacks,
    WeightedSquatJacks,
    WeightedTripleUnder,
    UnknownValue(u64),
}

impl From<FieldContent> for CardioExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => CardioExerciseName::BobAndWeaveCircle,
                1 => CardioExerciseName::WeightedBobAndWeaveCircle,
                2 => CardioExerciseName::CardioCoreCrawl,
                3 => CardioExerciseName::WeightedCardioCoreCrawl,
                4 => CardioExerciseName::DoubleUnder,
                5 => CardioExerciseName::WeightedDoubleUnder,
                6 => CardioExerciseName::JumpRope,
                7 => CardioExerciseName::WeightedJumpRope,
                8 => CardioExerciseName::JumpRopeCrossover,
                9 => CardioExerciseName::WeightedJumpRopeCrossover,
                10 => CardioExerciseName::JumpRopeJog,
                11 => CardioExerciseName::WeightedJumpRopeJog,
                12 => CardioExerciseName::JumpingJacks,
                13 => CardioExerciseName::WeightedJumpingJacks,
                14 => CardioExerciseName::SkiMoguls,
                15 => CardioExerciseName::WeightedSkiMoguls,
                16 => CardioExerciseName::SplitJacks,
                17 => CardioExerciseName::WeightedSplitJacks,
                18 => CardioExerciseName::SquatJacks,
                19 => CardioExerciseName::WeightedSquatJacks,
                20 => CardioExerciseName::TripleUnder,
                21 => CardioExerciseName::WeightedTripleUnder,
                n => CardioExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CardioExerciseName to {:?}", field);
        }
    }
}
