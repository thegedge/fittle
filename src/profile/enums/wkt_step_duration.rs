use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WktStepDuration {
    Calories,
    Distance,
    HrGreaterThan,
    HrLessThan,
    Open,
    Power10SGreaterThan,
    Power10SLessThan,
    Power30SGreaterThan,
    Power30SLessThan,
    Power3SGreaterThan,
    Power3SLessThan,
    PowerGreaterThan,
    PowerLapGreaterThan,
    PowerLapLessThan,
    PowerLessThan,
    RepeatUntilCalories,
    RepeatUntilDistance,
    RepeatUntilHrGreaterThan,
    RepeatUntilHrLessThan,
    RepeatUntilMaxPowerLastLapLessThan,
    RepeatUntilPowerGreaterThan,
    RepeatUntilPowerLastLapLessThan,
    RepeatUntilPowerLessThan,
    RepeatUntilStepsCmplt,
    RepeatUntilTime,
    RepeatUntilTrainingPeaksTss,
    RepetitionTime,
    Reps,
    Time,
    TrainingPeaksTss,
    UnknownValue(u64),
}

impl From<FieldContent> for WktStepDuration {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WktStepDuration::Time,
                1 => WktStepDuration::Distance,
                2 => WktStepDuration::HrLessThan,
                3 => WktStepDuration::HrGreaterThan,
                4 => WktStepDuration::Calories,
                5 => WktStepDuration::Open,
                6 => WktStepDuration::RepeatUntilStepsCmplt,
                7 => WktStepDuration::RepeatUntilTime,
                8 => WktStepDuration::RepeatUntilDistance,
                9 => WktStepDuration::RepeatUntilCalories,
                10 => WktStepDuration::RepeatUntilHrLessThan,
                11 => WktStepDuration::RepeatUntilHrGreaterThan,
                12 => WktStepDuration::RepeatUntilPowerLessThan,
                13 => WktStepDuration::RepeatUntilPowerGreaterThan,
                14 => WktStepDuration::PowerLessThan,
                15 => WktStepDuration::PowerGreaterThan,
                16 => WktStepDuration::TrainingPeaksTss,
                17 => WktStepDuration::RepeatUntilPowerLastLapLessThan,
                18 => WktStepDuration::RepeatUntilMaxPowerLastLapLessThan,
                19 => WktStepDuration::Power3SLessThan,
                20 => WktStepDuration::Power10SLessThan,
                21 => WktStepDuration::Power30SLessThan,
                22 => WktStepDuration::Power3SGreaterThan,
                23 => WktStepDuration::Power10SGreaterThan,
                24 => WktStepDuration::Power30SGreaterThan,
                25 => WktStepDuration::PowerLapLessThan,
                26 => WktStepDuration::PowerLapGreaterThan,
                27 => WktStepDuration::RepeatUntilTrainingPeaksTss,
                28 => WktStepDuration::RepetitionTime,
                29 => WktStepDuration::Reps,
                n => WktStepDuration::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WktStepDuration to {:?}", field);
        }
    }
}
