use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayOrientation {
    Auto,
    Landscape,
    LandscapeFlipped,
    Portrait,
    PortraitFlipped,
    UnknownValue(u64),
}

impl From<FieldContent> for DisplayOrientation {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DisplayOrientation::Auto,
                1 => DisplayOrientation::Portrait,
                2 => DisplayOrientation::Landscape,
                3 => DisplayOrientation::PortraitFlipped,
                4 => DisplayOrientation::LandscapeFlipped,
                n => DisplayOrientation::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DisplayOrientation to {:?}", field);
        }
    }
}
