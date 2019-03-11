use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherSeverity {
    Advisory,
    Statement,
    Unknown,
    Warning,
    Watch,
    UnknownValue(u64),
}

impl From<FieldContent> for WeatherSeverity {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WeatherSeverity::Unknown,
                1 => WeatherSeverity::Warning,
                2 => WeatherSeverity::Watch,
                3 => WeatherSeverity::Advisory,
                4 => WeatherSeverity::Statement,
                n => WeatherSeverity::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WeatherSeverity to {:?}", field);
        }
    }
}
