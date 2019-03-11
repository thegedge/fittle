use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaterType {
    Custom,
    En13319,
    Fresh,
    Salt,
    UnknownValue(u64),
}

impl From<FieldContent> for WaterType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WaterType::Fresh,
                1 => WaterType::Salt,
                2 => WaterType::En13319,
                3 => WaterType::Custom,
                n => WaterType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WaterType to {:?}", field);
        }
    }
}
