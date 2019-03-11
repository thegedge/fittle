use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunExerciseName {
    Jog,
    Run,
    Sprint,
    Walk,
    UnknownValue(u64),
}

impl From<FieldContent> for RunExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => RunExerciseName::Run,
                1 => RunExerciseName::Walk,
                2 => RunExerciseName::Jog,
                3 => RunExerciseName::Sprint,
                n => RunExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert RunExerciseName to {:?}", field);
        }
    }
}
