use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegCurlExerciseName {
    GoodMorning,
    LegCurl,
    SeatedBarbellGoodMorning,
    SingleLegBarbellGoodMorning,
    SingleLegSlidingLegCurl,
    SlidingLegCurl,
    SplitBarbellGoodMorning,
    SplitStanceExtension,
    StaggeredStanceGoodMorning,
    SwissBallHipRaiseAndLegCurl,
    WeightedLegCurl,
    ZercherGoodMorning,
    UnknownValue(u64),
}

impl From<FieldContent> for LegCurlExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => LegCurlExerciseName::LegCurl,
                1 => LegCurlExerciseName::WeightedLegCurl,
                2 => LegCurlExerciseName::GoodMorning,
                3 => LegCurlExerciseName::SeatedBarbellGoodMorning,
                4 => LegCurlExerciseName::SingleLegBarbellGoodMorning,
                5 => LegCurlExerciseName::SingleLegSlidingLegCurl,
                6 => LegCurlExerciseName::SlidingLegCurl,
                7 => LegCurlExerciseName::SplitBarbellGoodMorning,
                8 => LegCurlExerciseName::SplitStanceExtension,
                9 => LegCurlExerciseName::StaggeredStanceGoodMorning,
                10 => LegCurlExerciseName::SwissBallHipRaiseAndLegCurl,
                11 => LegCurlExerciseName::ZercherGoodMorning,
                n => LegCurlExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LegCurlExerciseName to {:?}", field);
        }
    }
}
