use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SensorType {
    Accelerometer,
    Barometer,
    Compass,
    Gyroscope,
    UnknownValue(u64),
}

impl From<FieldContent> for SensorType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SensorType::Accelerometer,
                1 => SensorType::Gyroscope,
                2 => SensorType::Compass,
                3 => SensorType::Barometer,
                n => SensorType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SensorType to {:?}", field);
        }
    }
}
