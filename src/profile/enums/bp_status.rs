use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BpStatus {
    ErrorDataOutOfRange,
    ErrorIncompleteData,
    ErrorIrregularHeartRate,
    ErrorNoMeasurement,
    NoError,
    UnknownValue(u64),
}

impl From<FieldContent> for BpStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => BpStatus::NoError,
                1 => BpStatus::ErrorIncompleteData,
                2 => BpStatus::ErrorNoMeasurement,
                3 => BpStatus::ErrorDataOutOfRange,
                4 => BpStatus::ErrorIrregularHeartRate,
                n => BpStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BpStatus to {:?}", field);
        }
    }
}
