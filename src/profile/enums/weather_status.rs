use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherStatus {
    Clear,
    Cloudy,
    Fog,
    Hail,
    Hazy,
    HeavyRain,
    HeavyRainSnow,
    HeavySnow,
    LightRain,
    LightRainSnow,
    LightSnow,
    MostlyCloudy,
    PartlyCloudy,
    Rain,
    ScatteredShowers,
    ScatteredThunderstorms,
    Snow,
    Thunderstorms,
    UnknownPrecipitation,
    Windy,
    WintryMix,
    UnknownValue(u64),
}

impl From<FieldContent> for WeatherStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => WeatherStatus::Clear,
                1 => WeatherStatus::PartlyCloudy,
                2 => WeatherStatus::MostlyCloudy,
                3 => WeatherStatus::Rain,
                4 => WeatherStatus::Snow,
                5 => WeatherStatus::Windy,
                6 => WeatherStatus::Thunderstorms,
                7 => WeatherStatus::WintryMix,
                8 => WeatherStatus::Fog,
                11 => WeatherStatus::Hazy,
                12 => WeatherStatus::Hail,
                13 => WeatherStatus::ScatteredShowers,
                14 => WeatherStatus::ScatteredThunderstorms,
                15 => WeatherStatus::UnknownPrecipitation,
                16 => WeatherStatus::LightRain,
                17 => WeatherStatus::HeavyRain,
                18 => WeatherStatus::LightSnow,
                19 => WeatherStatus::HeavySnow,
                20 => WeatherStatus::LightRainSnow,
                21 => WeatherStatus::HeavyRainSnow,
                22 => WeatherStatus::Cloudy,
                n => WeatherStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert WeatherStatus to {:?}", field);
        }
    }
}
