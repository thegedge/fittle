use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HipSwingExerciseName {
    SingleArmDumbbellSwing,
    SingleArmKettlebellSwing,
    StepOutSwing,
    UnknownValue(u64),
}

impl From<FieldContent> for HipSwingExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => HipSwingExerciseName::SingleArmKettlebellSwing,
                1 => HipSwingExerciseName::SingleArmDumbbellSwing,
                2 => HipSwingExerciseName::StepOutSwing,
                n => HipSwingExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert HipSwingExerciseName to {:?}", field);
        }
    }
}
