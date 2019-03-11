use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FitBaseUnit {
    Kilogram,
    Other,
    Pound,
    UnknownValue(u64),
}

impl From<FieldContent> for FitBaseUnit {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => FitBaseUnit::Other,
                1 => FitBaseUnit::Kilogram,
                2 => FitBaseUnit::Pound,
                n => FitBaseUnit::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FitBaseUnit to {:?}", field);
        }
    }
}
