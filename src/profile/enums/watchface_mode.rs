use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchfaceMode {
    Analog,
    ConnectIq,
    Digital,
    Disabled,
    UnknownValue(u64),
}

impl From<FieldContent> for WatchfaceMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WatchfaceMode::Digital,
                1 => WatchfaceMode::Analog,
                2 => WatchfaceMode::ConnectIq,
                3 => WatchfaceMode::Disabled,
                n => WatchfaceMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WatchfaceMode to {:?}", field);
        }
    }
}
