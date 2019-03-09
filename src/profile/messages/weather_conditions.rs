// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct WeatherConditions {
    timestamp: Option<enums::DateTime>,
    weather_report: Option<enums::WeatherReport>,
    temperature: Option<i8>,
    condition: Option<enums::WeatherStatus>,
    wind_direction: Option<u16>,
    wind_speed: Option<u16>,
    precipitation_probability: Option<u8>,
    temperature_feels_like: Option<i8>,
    relative_humidity: Option<u8>,
    location: Option<String>,
    observed_at_time: Option<enums::DateTime>,
    observed_location_lat: Option<i32>,
    observed_location_long: Option<i32>,
    day_of_week: Option<enums::DayOfWeek>,
    high_temperature: Option<i8>,
    low_temperature: Option<i8>,
}

impl From<Vec<(u8, Field)>> for WeatherConditions {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.weather_report = field.one().map(<enums::WeatherReport>::from),
                1 => msg.temperature = field.one().map(<i8>::from),
                2 => msg.condition = field.one().map(<enums::WeatherStatus>::from),
                3 => msg.wind_direction = field.one().map(<u16>::from),
                4 => msg.wind_speed = field.one().map(<u16>::from),
                5 => msg.precipitation_probability = field.one().map(<u8>::from),
                6 => msg.temperature_feels_like = field.one().map(<i8>::from),
                7 => msg.relative_humidity = field.one().map(<u8>::from),
                8 => msg.location = field.one().map(<String>::from),
                9 => msg.observed_at_time = field.one().map(<enums::DateTime>::from),
                10 => msg.observed_location_lat = field.one().map(<i32>::from),
                11 => msg.observed_location_long = field.one().map(<i32>::from),
                12 => msg.day_of_week = field.one().map(<enums::DayOfWeek>::from),
                13 => msg.high_temperature = field.one().map(<i8>::from),
                14 => msg.low_temperature = field.one().map(<i8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

