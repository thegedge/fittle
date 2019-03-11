use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyeExerciseName {
    ArmRotations,
    CableCrossover,
    DeclineDumbbellFlye,
    DumbbellFlye,
    HugATree,
    InclineDumbbellFlye,
    KettlebellFlye,
    KneelingRearFlye,
    SingleArmStandingCableReverseFlye,
    SwissBallDumbbellFlye,
    UnknownValue(u64),
}

impl From<FieldContent> for FlyeExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => FlyeExerciseName::CableCrossover,
                1 => FlyeExerciseName::DeclineDumbbellFlye,
                2 => FlyeExerciseName::DumbbellFlye,
                3 => FlyeExerciseName::InclineDumbbellFlye,
                4 => FlyeExerciseName::KettlebellFlye,
                5 => FlyeExerciseName::KneelingRearFlye,
                6 => FlyeExerciseName::SingleArmStandingCableReverseFlye,
                7 => FlyeExerciseName::SwissBallDumbbellFlye,
                8 => FlyeExerciseName::ArmRotations,
                9 => FlyeExerciseName::HugATree,
                n => FlyeExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FlyeExerciseName to {:?}", field);
        }
    }
}
