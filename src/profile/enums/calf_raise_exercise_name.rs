use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CalfRaiseExerciseName {
    DonkeyCalfRaise,
    SeatedCalfRaise,
    SeatedDumbbellToeRaise,
    SingleLegBentKneeCalfRaise,
    SingleLegDeclinePushUp,
    SingleLegDonkeyCalfRaise,
    SingleLegHipRaiseWithKneeHold,
    SingleLegStandingCalfRaise,
    SingleLegStandingDumbbellCalfRaise,
    StandingBarbellCalfRaise,
    StandingCalfRaise,
    StandingDumbbellCalfRaise,
    ThreeWayCalfRaise,
    ThreeWaySingleLegCalfRaise,
    ThreeWayWeightedCalfRaise,
    ThreeWayWeightedSingleLegCalfRaise,
    WeightedDonkeyCalfRaise,
    WeightedSeatedCalfRaise,
    WeightedSingleLegBentKneeCalfRaise,
    WeightedSingleLegDonkeyCalfRaise,
    WeightedStandingCalfRaise,
    UnknownValue(u64),
}

impl From<FieldContent> for CalfRaiseExerciseName {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => CalfRaiseExerciseName::ThreeWayCalfRaise,
                1 => CalfRaiseExerciseName::ThreeWayWeightedCalfRaise,
                2 => CalfRaiseExerciseName::ThreeWaySingleLegCalfRaise,
                3 => CalfRaiseExerciseName::ThreeWayWeightedSingleLegCalfRaise,
                4 => CalfRaiseExerciseName::DonkeyCalfRaise,
                5 => CalfRaiseExerciseName::WeightedDonkeyCalfRaise,
                6 => CalfRaiseExerciseName::SeatedCalfRaise,
                7 => CalfRaiseExerciseName::WeightedSeatedCalfRaise,
                8 => CalfRaiseExerciseName::SeatedDumbbellToeRaise,
                9 => CalfRaiseExerciseName::SingleLegBentKneeCalfRaise,
                10 => CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise,
                11 => CalfRaiseExerciseName::SingleLegDeclinePushUp,
                12 => CalfRaiseExerciseName::SingleLegDonkeyCalfRaise,
                13 => CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise,
                14 => CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold,
                15 => CalfRaiseExerciseName::SingleLegStandingCalfRaise,
                16 => CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise,
                17 => CalfRaiseExerciseName::StandingBarbellCalfRaise,
                18 => CalfRaiseExerciseName::StandingCalfRaise,
                19 => CalfRaiseExerciseName::WeightedStandingCalfRaise,
                20 => CalfRaiseExerciseName::StandingDumbbellCalfRaise,
                n => CalfRaiseExerciseName::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CalfRaiseExerciseName to {:?}", field);
        }
    }
}
