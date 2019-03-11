use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShrugExerciseName {
    BarbellJumpShrug,
    BarbellShrug,
    BarbellUprightRow,
    BehindTheBackSmithMachineShrug,
    DumbbellJumpShrug,
    DumbbellShrug,
    DumbbellUprightRow,
    InclineDumbbellShrug,
    OverheadBarbellShrug,
    OverheadDumbbellShrug,
    ScaptionAndShrug,
    ScapularRetraction,
    SerratusChairShrug,
    SerratusShrug,
    WeightedSerratusChairShrug,
    WeightedSerratusShrug,
    WideGripJumpShrug,
    UnknownValue(u64),
}

impl From<FieldContent> for ShrugExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => ShrugExerciseName::BarbellJumpShrug,
                1 => ShrugExerciseName::BarbellShrug,
                2 => ShrugExerciseName::BarbellUprightRow,
                3 => ShrugExerciseName::BehindTheBackSmithMachineShrug,
                4 => ShrugExerciseName::DumbbellJumpShrug,
                5 => ShrugExerciseName::DumbbellShrug,
                6 => ShrugExerciseName::DumbbellUprightRow,
                7 => ShrugExerciseName::InclineDumbbellShrug,
                8 => ShrugExerciseName::OverheadBarbellShrug,
                9 => ShrugExerciseName::OverheadDumbbellShrug,
                10 => ShrugExerciseName::ScaptionAndShrug,
                11 => ShrugExerciseName::ScapularRetraction,
                12 => ShrugExerciseName::SerratusChairShrug,
                13 => ShrugExerciseName::WeightedSerratusChairShrug,
                14 => ShrugExerciseName::SerratusShrug,
                15 => ShrugExerciseName::WeightedSerratusShrug,
                16 => ShrugExerciseName::WideGripJumpShrug,
                n => ShrugExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ShrugExerciseName to {:?}", field);
        }
    }
}
