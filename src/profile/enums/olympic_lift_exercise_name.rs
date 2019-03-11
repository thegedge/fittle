use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OlympicLiftExerciseName {
    BarbellHangPowerClean,
    BarbellHangPowerSnatch,
    BarbellHangPull,
    BarbellHangSquatClean,
    BarbellHighPull,
    BarbellPowerClean,
    BarbellPowerSnatch,
    BarbellSnatch,
    BarbellSplitJerk,
    BarbellSquatClean,
    Clean,
    CleanAndJerk,
    DumbbellClean,
    DumbbellHangPull,
    OneHandDumbbellSplitSnatch,
    PushJerk,
    SingleArmDumbbellSnatch,
    SingleArmHangSnatch,
    SingleArmKettlebellSnatch,
    SplitJerk,
    SquatCleanAndJerk,
    UnknownValue(u64),
}

impl From<FieldContent> for OlympicLiftExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => OlympicLiftExerciseName::BarbellHangPowerClean,
                1 => OlympicLiftExerciseName::BarbellHangSquatClean,
                2 => OlympicLiftExerciseName::BarbellPowerClean,
                3 => OlympicLiftExerciseName::BarbellPowerSnatch,
                4 => OlympicLiftExerciseName::BarbellSquatClean,
                5 => OlympicLiftExerciseName::CleanAndJerk,
                6 => OlympicLiftExerciseName::BarbellHangPowerSnatch,
                7 => OlympicLiftExerciseName::BarbellHangPull,
                8 => OlympicLiftExerciseName::BarbellHighPull,
                9 => OlympicLiftExerciseName::BarbellSnatch,
                10 => OlympicLiftExerciseName::BarbellSplitJerk,
                11 => OlympicLiftExerciseName::Clean,
                12 => OlympicLiftExerciseName::DumbbellClean,
                13 => OlympicLiftExerciseName::DumbbellHangPull,
                14 => OlympicLiftExerciseName::OneHandDumbbellSplitSnatch,
                15 => OlympicLiftExerciseName::PushJerk,
                16 => OlympicLiftExerciseName::SingleArmDumbbellSnatch,
                17 => OlympicLiftExerciseName::SingleArmHangSnatch,
                18 => OlympicLiftExerciseName::SingleArmKettlebellSnatch,
                19 => OlympicLiftExerciseName::SplitJerk,
                20 => OlympicLiftExerciseName::SquatCleanAndJerk,
                n => OlympicLiftExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert OlympicLiftExerciseName to {:?}", field);
        }
    }
}
