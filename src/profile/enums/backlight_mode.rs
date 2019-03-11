use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BacklightMode {
    AutoBrightness,
    KeyAndMessages,
    KeyAndMessagesAndSmartNotifications,
    KeyAndMessagesNight,
    Manual,
    Off,
    SmartNotifications,
    UnknownValue(u64),
}

impl From<FieldContent> for BacklightMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => BacklightMode::Off,
                1 => BacklightMode::Manual,
                2 => BacklightMode::KeyAndMessages,
                3 => BacklightMode::AutoBrightness,
                4 => BacklightMode::SmartNotifications,
                5 => BacklightMode::KeyAndMessagesNight,
                6 => BacklightMode::KeyAndMessagesAndSmartNotifications,
                n => BacklightMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BacklightMode to {:?}", field);
        }
    }
}
