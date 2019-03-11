use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalWatchfaceLayout {
    Bold,
    Modern,
    Traditional,
    UnknownValue(u64),
}

impl From<FieldContent> for DigitalWatchfaceLayout {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DigitalWatchfaceLayout::Traditional,
                1 => DigitalWatchfaceLayout::Modern,
                2 => DigitalWatchfaceLayout::Bold,
                n => DigitalWatchfaceLayout::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DigitalWatchfaceLayout to {:?}", field);
        }
    }
}
