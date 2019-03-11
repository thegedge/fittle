use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShoulderPressExerciseName {
    AlternatingDumbbellShoulderPress,
    ArnoldPress,
    BarbellFrontSquatToPushPress,
    BarbellPushPress,
    BarbellShoulderPress,
    DeadCurlPress,
    DumbbellAlternatingShoulderPressAndTwist,
    DumbbellHammerCurlToLungeToPress,
    DumbbellPushPress,
    FloorInvertedShoulderPress,
    InvertedShoulderPress,
    OneArmPushPress,
    OverheadBarbellPress,
    OverheadDumbbellPress,
    SeatedBarbellShoulderPress,
    SeatedDumbbellShoulderPress,
    SingleArmDumbbellShoulderPress,
    SingleArmStepUpAndPress,
    SmithMachineOverheadPress,
    SplitStanceHammerCurlToPress,
    SwissBallDumbbellShoulderPress,
    WeightPlateFrontRaise,
    WeightedFloorInvertedShoulderPress,
    WeightedInvertedShoulderPress,
    UnknownValue(u64),
}

impl From<FieldContent> for ShoulderPressExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => ShoulderPressExerciseName::AlternatingDumbbellShoulderPress,
                1 => ShoulderPressExerciseName::ArnoldPress,
                2 => ShoulderPressExerciseName::BarbellFrontSquatToPushPress,
                3 => ShoulderPressExerciseName::BarbellPushPress,
                4 => ShoulderPressExerciseName::BarbellShoulderPress,
                5 => ShoulderPressExerciseName::DeadCurlPress,
                6 => ShoulderPressExerciseName::DumbbellAlternatingShoulderPressAndTwist,
                7 => ShoulderPressExerciseName::DumbbellHammerCurlToLungeToPress,
                8 => ShoulderPressExerciseName::DumbbellPushPress,
                9 => ShoulderPressExerciseName::FloorInvertedShoulderPress,
                10 => ShoulderPressExerciseName::WeightedFloorInvertedShoulderPress,
                11 => ShoulderPressExerciseName::InvertedShoulderPress,
                12 => ShoulderPressExerciseName::WeightedInvertedShoulderPress,
                13 => ShoulderPressExerciseName::OneArmPushPress,
                14 => ShoulderPressExerciseName::OverheadBarbellPress,
                15 => ShoulderPressExerciseName::OverheadDumbbellPress,
                16 => ShoulderPressExerciseName::SeatedBarbellShoulderPress,
                17 => ShoulderPressExerciseName::SeatedDumbbellShoulderPress,
                18 => ShoulderPressExerciseName::SingleArmDumbbellShoulderPress,
                19 => ShoulderPressExerciseName::SingleArmStepUpAndPress,
                20 => ShoulderPressExerciseName::SmithMachineOverheadPress,
                21 => ShoulderPressExerciseName::SplitStanceHammerCurlToPress,
                22 => ShoulderPressExerciseName::SwissBallDumbbellShoulderPress,
                23 => ShoulderPressExerciseName::WeightPlateFrontRaise,
                n => ShoulderPressExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ShoulderPressExerciseName to {:?}", field);
        }
    }
}
