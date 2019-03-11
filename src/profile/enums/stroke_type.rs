use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StrokeType {
    Backhand,
    Forehand,
    NoEvent,
    Other,
    Serve,
    Smash,
    UnknownValue(u64),
}

impl From<FieldContent> for StrokeType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => StrokeType::NoEvent,
                1 => StrokeType::Other,
                2 => StrokeType::Serve,
                3 => StrokeType::Forehand,
                4 => StrokeType::Backhand,
                5 => StrokeType::Smash,
                n => StrokeType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert StrokeType to {:?}", field);
        }
    }
}
