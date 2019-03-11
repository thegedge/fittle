use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayHeart {
    Bpm,
    Max,
    Reserve,
    UnknownValue(u64),
}

impl From<FieldContent> for DisplayHeart {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DisplayHeart::Bpm,
                1 => DisplayHeart::Max,
                2 => DisplayHeart::Reserve,
                n => DisplayHeart::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DisplayHeart to {:?}", field);
        }
    }
}
