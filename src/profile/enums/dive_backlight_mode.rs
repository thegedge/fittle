use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiveBacklightMode {
    AlwaysOn,
    AtDepth,
    UnknownValue(u64),
}

impl From<FieldContent> for DiveBacklightMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DiveBacklightMode::AtDepth,
                1 => DiveBacklightMode::AlwaysOn,
                n => DiveBacklightMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DiveBacklightMode to {:?}", field);
        }
    }
}
