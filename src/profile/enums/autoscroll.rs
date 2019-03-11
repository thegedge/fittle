use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Autoscroll {
    Fast,
    Medium,
    None,
    Slow,
    UnknownValue(u64),
}

impl From<FieldContent> for Autoscroll {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Autoscroll::None,
                1 => Autoscroll::Slow,
                2 => Autoscroll::Medium,
                3 => Autoscroll::Fast,
                n => Autoscroll::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Autoscroll to {:?}", field);
        }
    }
}
