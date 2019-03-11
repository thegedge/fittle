use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Switch {
    Auto,
    Off,
    On,
    UnknownValue(u64),
}

impl From<FieldContent> for Switch {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Switch::Off,
                1 => Switch::On,
                2 => Switch::Auto,
                n => Switch::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Switch to {:?}", field);
        }
    }
}
