use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchPressExerciseName {
    AlternatingDumbbellChestPress,
    AlternatingDumbbellChestPressOnSwissBall,
    BarbellBenchPress,
    BarbellBoardBenchPress,
    BarbellFloorPress,
    CloseGripBarbellBenchPress,
    DeclineDumbbellBenchPress,
    DumbbellBenchPress,
    DumbbellFloorPress,
    InclineBarbellBenchPress,
    InclineDumbbellBenchPress,
    InclineSmithMachineBenchPress,
    IsometricBarbellBenchPress,
    KettlebellChestPress,
    NeutralGripDumbbellBenchPress,
    NeutralGripDumbbellInclineBenchPress,
    OneArmFloorPress,
    PartialLockout,
    ReverseGripBarbellBenchPress,
    ReverseGripInclineBenchPress,
    SingleArmCableChestPress,
    SingleArmDumbbellBenchPress,
    SmithMachineBenchPress,
    SwissBallDumbbellChestPress,
    TripleStopBarbellBenchPress,
    WeightedOneArmFloorPress,
    WideGripBarbellBenchPress,
    UnknownValue(u64),
}

impl From<FieldContent> for BenchPressExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => BenchPressExerciseName::AlternatingDumbbellChestPressOnSwissBall,
                1 => BenchPressExerciseName::BarbellBenchPress,
                2 => BenchPressExerciseName::BarbellBoardBenchPress,
                3 => BenchPressExerciseName::BarbellFloorPress,
                4 => BenchPressExerciseName::CloseGripBarbellBenchPress,
                5 => BenchPressExerciseName::DeclineDumbbellBenchPress,
                6 => BenchPressExerciseName::DumbbellBenchPress,
                7 => BenchPressExerciseName::DumbbellFloorPress,
                8 => BenchPressExerciseName::InclineBarbellBenchPress,
                9 => BenchPressExerciseName::InclineDumbbellBenchPress,
                10 => BenchPressExerciseName::InclineSmithMachineBenchPress,
                11 => BenchPressExerciseName::IsometricBarbellBenchPress,
                12 => BenchPressExerciseName::KettlebellChestPress,
                13 => BenchPressExerciseName::NeutralGripDumbbellBenchPress,
                14 => BenchPressExerciseName::NeutralGripDumbbellInclineBenchPress,
                15 => BenchPressExerciseName::OneArmFloorPress,
                16 => BenchPressExerciseName::WeightedOneArmFloorPress,
                17 => BenchPressExerciseName::PartialLockout,
                18 => BenchPressExerciseName::ReverseGripBarbellBenchPress,
                19 => BenchPressExerciseName::ReverseGripInclineBenchPress,
                20 => BenchPressExerciseName::SingleArmCableChestPress,
                21 => BenchPressExerciseName::SingleArmDumbbellBenchPress,
                22 => BenchPressExerciseName::SmithMachineBenchPress,
                23 => BenchPressExerciseName::SwissBallDumbbellChestPress,
                24 => BenchPressExerciseName::TripleStopBarbellBenchPress,
                25 => BenchPressExerciseName::WideGripBarbellBenchPress,
                26 => BenchPressExerciseName::AlternatingDumbbellChestPress,
                n => BenchPressExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BenchPressExerciseName to {:?}", field);
        }
    }
}
