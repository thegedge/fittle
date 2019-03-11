use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    BeginDepreciated,
    ConsecutiveDepreciated,
    EndAllDepreciated,
    EndDepreciated,
    Marker,
    Start,
    Stop,
    StopAll,
    StopDisable,
    StopDisableAll,
    UnknownValue(u64),
}

impl From<FieldContent> for EventType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => EventType::Start,
                1 => EventType::Stop,
                2 => EventType::ConsecutiveDepreciated,
                3 => EventType::Marker,
                4 => EventType::StopAll,
                5 => EventType::BeginDepreciated,
                6 => EventType::EndDepreciated,
                7 => EventType::EndAllDepreciated,
                8 => EventType::StopDisable,
                9 => EventType::StopDisableAll,
                n => EventType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert EventType to {:?}", field);
        }
    }
}
