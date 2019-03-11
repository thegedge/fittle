use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseCategory {
    BenchPress,
    CalfRaise,
    Cardio,
    Carry,
    Chop,
    Core,
    Crunch,
    Curl,
    Deadlift,
    Flye,
    HipRaise,
    HipStability,
    HipSwing,
    Hyperextension,
    LateralRaise,
    LegCurl,
    LegRaise,
    Lunge,
    OlympicLift,
    Plank,
    Plyo,
    PullUp,
    PushUp,
    Row,
    Run,
    ShoulderPress,
    ShoulderStability,
    Shrug,
    SitUp,
    Squat,
    TotalBody,
    TricepsExtension,
    Unknown,
    WarmUp,
    UnknownValue(u64),
}

impl From<FieldContent> for ExerciseCategory {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => ExerciseCategory::BenchPress,
                1 => ExerciseCategory::CalfRaise,
                2 => ExerciseCategory::Cardio,
                3 => ExerciseCategory::Carry,
                4 => ExerciseCategory::Chop,
                5 => ExerciseCategory::Core,
                6 => ExerciseCategory::Crunch,
                7 => ExerciseCategory::Curl,
                8 => ExerciseCategory::Deadlift,
                9 => ExerciseCategory::Flye,
                10 => ExerciseCategory::HipRaise,
                11 => ExerciseCategory::HipStability,
                12 => ExerciseCategory::HipSwing,
                13 => ExerciseCategory::Hyperextension,
                14 => ExerciseCategory::LateralRaise,
                15 => ExerciseCategory::LegCurl,
                16 => ExerciseCategory::LegRaise,
                17 => ExerciseCategory::Lunge,
                18 => ExerciseCategory::OlympicLift,
                19 => ExerciseCategory::Plank,
                20 => ExerciseCategory::Plyo,
                21 => ExerciseCategory::PullUp,
                22 => ExerciseCategory::PushUp,
                23 => ExerciseCategory::Row,
                24 => ExerciseCategory::ShoulderPress,
                25 => ExerciseCategory::ShoulderStability,
                26 => ExerciseCategory::Shrug,
                27 => ExerciseCategory::SitUp,
                28 => ExerciseCategory::Squat,
                29 => ExerciseCategory::TotalBody,
                30 => ExerciseCategory::TricepsExtension,
                31 => ExerciseCategory::WarmUp,
                32 => ExerciseCategory::Run,
                65534 => ExerciseCategory::Unknown,
                n => ExerciseCategory::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ExerciseCategory to {:?}", field);
        }
    }
}
