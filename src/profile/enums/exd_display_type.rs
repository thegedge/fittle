use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExdDisplayType {
    Balance,
    Bar,
    CircleGraph,
    Gauge,
    Graph,
    Numerical,
    Simple,
    SimpleDynamicIcon,
    String,
    StringList,
    VirtualPartner,
    UnknownValue(u64),
}

impl From<FieldContent> for ExdDisplayType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ExdDisplayType::Numerical,
                1 => ExdDisplayType::Simple,
                2 => ExdDisplayType::Graph,
                3 => ExdDisplayType::Bar,
                4 => ExdDisplayType::CircleGraph,
                5 => ExdDisplayType::VirtualPartner,
                6 => ExdDisplayType::Balance,
                7 => ExdDisplayType::StringList,
                8 => ExdDisplayType::String,
                9 => ExdDisplayType::SimpleDynamicIcon,
                10 => ExdDisplayType::Gauge,
                n => ExdDisplayType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ExdDisplayType to {:?}", field);
        }
    }
}
