// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl WeatherConditions {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.weather_report = content.one().map(<enums::WeatherReport>::from),
                1 => msg.temperature = content.one().map(<i8>::from),
                2 => msg.condition = content.one().map(<enums::WeatherStatus>::from),
                3 => msg.wind_direction = content.one().map(<u16>::from),
                4 => msg.wind_speed = content.one().map(<u16>::from),
                5 => msg.precipitation_probability = content.one().map(<u8>::from),
                6 => msg.temperature_feels_like = content.one().map(<i8>::from),
                7 => msg.relative_humidity = content.one().map(<u8>::from),
                8 => msg.location = content.one().map(<String>::from),
                9 => msg.observed_at_time = content.one().map(<enums::DateTime>::from),
                10 => msg.observed_location_lat = content.one().map(<i32>::from),
                11 => msg.observed_location_long = content.one().map(<i32>::from),
                12 => msg.day_of_week = content.one().map(<enums::DayOfWeek>::from),
                13 => msg.high_temperature = content.one().map(<i8>::from),
                14 => msg.low_temperature = content.one().map(<i8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

