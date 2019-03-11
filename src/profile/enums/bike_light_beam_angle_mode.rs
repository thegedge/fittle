use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BikeLightBeamAngleMode {
    Auto,
    Manual,
    UnknownValue(u64),
}

impl From<FieldContent> for BikeLightBeamAngleMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                0 => BikeLightBeamAngleMode::Manual,
                1 => BikeLightBeamAngleMode::Auto,
                n => BikeLightBeamAngleMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BikeLightBeamAngleMode to {:?}", field);
        }
    }
}
