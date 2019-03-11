use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalogWatchfaceLayout {
    Minimal,
    Modern,
    Traditional,
    UnknownValue(u64),
}

impl From<FieldContent> for AnalogWatchfaceLayout {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => AnalogWatchfaceLayout::Minimal,
                1 => AnalogWatchfaceLayout::Traditional,
                2 => AnalogWatchfaceLayout::Modern,
                n => AnalogWatchfaceLayout::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AnalogWatchfaceLayout to {:?}", field);
        }
    }
}
