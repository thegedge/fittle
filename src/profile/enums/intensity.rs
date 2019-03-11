use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Intensity {
    Active,
    Cooldown,
    Rest,
    Warmup,
    UnknownValue(u64),
}

impl From<FieldContent> for Intensity {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Intensity::Active,
                1 => Intensity::Rest,
                2 => Intensity::Warmup,
                3 => Intensity::Cooldown,
                n => Intensity::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Intensity to {:?}", field);
        }
    }
}
