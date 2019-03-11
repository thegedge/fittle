use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RowExerciseName {
    BarbellStraightLegDeadliftToRow,
    CableRowStanding,
    DumbbellRow,
    ElevatedFeetInvertedRow,
    FacePull,
    FacePullWithExternalRotation,
    InvertedRowWithFeetOnSwissBall,
    KettlebellRow,
    ModifiedInvertedRow,
    NeutralGripAlternatingDumbbellRow,
    OneArmBentOverRow,
    OneLeggedDumbbellRow,
    RenegadeRow,
    ReverseGripBarbellRow,
    RopeHandleCableRow,
    SeatedCableRow,
    SeatedDumbbellRow,
    SingleArmCableRow,
    SingleArmCableRowAndRotation,
    SingleArmInvertedRow,
    SingleArmNeutralGripDumbbellRow,
    SingleArmNeutralGripDumbbellRowAndRotation,
    SuspendedInvertedRow,
    TBarRow,
    TowelGripInvertedRow,
    UnderhandGripCableRow,
    VGripCableRow,
    WeightedElevatedFeetInvertedRow,
    WeightedInvertedRowWithFeetOnSwissBall,
    WeightedModifiedInvertedRow,
    WeightedSingleArmInvertedRow,
    WeightedSuspendedInvertedRow,
    WeightedTowelGripInvertedRow,
    WideGripSeatedCableRow,
    UnknownValue(u64),
}

impl From<FieldContent> for RowExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => RowExerciseName::BarbellStraightLegDeadliftToRow,
                1 => RowExerciseName::CableRowStanding,
                2 => RowExerciseName::DumbbellRow,
                3 => RowExerciseName::ElevatedFeetInvertedRow,
                4 => RowExerciseName::WeightedElevatedFeetInvertedRow,
                5 => RowExerciseName::FacePull,
                6 => RowExerciseName::FacePullWithExternalRotation,
                7 => RowExerciseName::InvertedRowWithFeetOnSwissBall,
                8 => RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall,
                9 => RowExerciseName::KettlebellRow,
                10 => RowExerciseName::ModifiedInvertedRow,
                11 => RowExerciseName::WeightedModifiedInvertedRow,
                12 => RowExerciseName::NeutralGripAlternatingDumbbellRow,
                13 => RowExerciseName::OneArmBentOverRow,
                14 => RowExerciseName::OneLeggedDumbbellRow,
                15 => RowExerciseName::RenegadeRow,
                16 => RowExerciseName::ReverseGripBarbellRow,
                17 => RowExerciseName::RopeHandleCableRow,
                18 => RowExerciseName::SeatedCableRow,
                19 => RowExerciseName::SeatedDumbbellRow,
                20 => RowExerciseName::SingleArmCableRow,
                21 => RowExerciseName::SingleArmCableRowAndRotation,
                22 => RowExerciseName::SingleArmInvertedRow,
                23 => RowExerciseName::WeightedSingleArmInvertedRow,
                24 => RowExerciseName::SingleArmNeutralGripDumbbellRow,
                25 => RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation,
                26 => RowExerciseName::SuspendedInvertedRow,
                27 => RowExerciseName::WeightedSuspendedInvertedRow,
                28 => RowExerciseName::TBarRow,
                29 => RowExerciseName::TowelGripInvertedRow,
                30 => RowExerciseName::WeightedTowelGripInvertedRow,
                31 => RowExerciseName::UnderhandGripCableRow,
                32 => RowExerciseName::VGripCableRow,
                33 => RowExerciseName::WideGripSeatedCableRow,
                n => RowExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert RowExerciseName to {:?}", field);
        }
    }
}
