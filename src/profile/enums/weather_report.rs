use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherReport {
    Current,
    DailyForecast,
    HourlyForecast,
    UnknownValue(u64),
}

impl From<FieldContent> for WeatherReport {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WeatherReport::Current,
                1 => WeatherReport::HourlyForecast,
                2 => WeatherReport::DailyForecast,
                n => WeatherReport::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WeatherReport to {:?}", field);
        }
    }
}
