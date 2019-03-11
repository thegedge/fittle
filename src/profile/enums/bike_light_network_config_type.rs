use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BikeLightNetworkConfigType {
    Auto,
    HighVisibility,
    Individual,
    Trail,
    UnknownValue(u64),
}

impl From<FieldContent> for BikeLightNetworkConfigType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => BikeLightNetworkConfigType::Auto,
                4 => BikeLightNetworkConfigType::Individual,
                5 => BikeLightNetworkConfigType::HighVisibility,
                6 => BikeLightNetworkConfigType::Trail,
                n => BikeLightNetworkConfigType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BikeLightNetworkConfigType to {:?}", field);
        }
    }
}
