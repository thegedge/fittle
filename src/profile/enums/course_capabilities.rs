use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseCapabilities {
    Bikeway,
    Cadence,
    Distance,
    HeartRate,
    Navigation,
    Position,
    Power,
    Processed,
    Time,
    Training,
    Valid,
    UnknownValue(u64),
}

impl From<FieldContent> for CourseCapabilities {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32z(enum_value) = field {
            match enum_value {
                1 => CourseCapabilities::Processed,
                2 => CourseCapabilities::Valid,
                4 => CourseCapabilities::Time,
                8 => CourseCapabilities::Distance,
                16 => CourseCapabilities::Position,
                32 => CourseCapabilities::HeartRate,
                64 => CourseCapabilities::Power,
                128 => CourseCapabilities::Cadence,
                256 => CourseCapabilities::Training,
                512 => CourseCapabilities::Navigation,
                1024 => CourseCapabilities::Bikeway,
                n => CourseCapabilities::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CourseCapabilities to {:?}", field);
        }
    }
}
