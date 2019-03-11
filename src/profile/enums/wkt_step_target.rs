use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WktStepTarget {
    Cadence,
    Grade,
    HeartRate,
    HeartRateLap,
    Open,
    Power,
    Power10S,
    Power30S,
    Power3S,
    PowerLap,
    Resistance,
    Speed,
    SpeedLap,
    SwimStroke,
    UnknownValue(u64),
}

impl From<FieldContent> for WktStepTarget {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WktStepTarget::Speed,
                1 => WktStepTarget::HeartRate,
                2 => WktStepTarget::Open,
                3 => WktStepTarget::Cadence,
                4 => WktStepTarget::Power,
                5 => WktStepTarget::Grade,
                6 => WktStepTarget::Resistance,
                7 => WktStepTarget::Power3S,
                8 => WktStepTarget::Power10S,
                9 => WktStepTarget::Power30S,
                10 => WktStepTarget::PowerLap,
                11 => WktStepTarget::SwimStroke,
                12 => WktStepTarget::SpeedLap,
                13 => WktStepTarget::HeartRateLap,
                n => WktStepTarget::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WktStepTarget to {:?}", field);
        }
    }
}
