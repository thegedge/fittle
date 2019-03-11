use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CarryExerciseName {
    BarHolds,
    FarmersWalk,
    FarmersWalkOnToes,
    HexDumbbellHold,
    OverheadCarry,
    UnknownValue(u64),
}

impl From<FieldContent> for CarryExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => CarryExerciseName::BarHolds,
                1 => CarryExerciseName::FarmersWalk,
                2 => CarryExerciseName::FarmersWalkOnToes,
                3 => CarryExerciseName::HexDumbbellHold,
                4 => CarryExerciseName::OverheadCarry,
                n => CarryExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CarryExerciseName to {:?}", field);
        }
    }
}
