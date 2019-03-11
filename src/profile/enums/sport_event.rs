use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SportEvent {
    Fitness,
    Geocaching,
    Race,
    Recreation,
    SpecialEvent,
    Touring,
    Training,
    Transportation,
    Uncategorized,
    UnknownValue(u64),
}

impl From<FieldContent> for SportEvent {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SportEvent::Uncategorized,
                1 => SportEvent::Geocaching,
                2 => SportEvent::Fitness,
                3 => SportEvent::Recreation,
                4 => SportEvent::Race,
                5 => SportEvent::SpecialEvent,
                6 => SportEvent::Training,
                7 => SportEvent::Transportation,
                8 => SportEvent::Touring,
                n => SportEvent::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SportEvent to {:?}", field);
        }
    }
}
