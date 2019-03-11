use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiveAlarmType {
    Depth,
    Time,
    UnknownValue(u64),
}

impl From<FieldContent> for DiveAlarmType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DiveAlarmType::Depth,
                1 => DiveAlarmType::Time,
                n => DiveAlarmType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DiveAlarmType to {:?}", field);
        }
    }
}
