// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct WeatherConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<enums::WeatherStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<enums::DayOfWeek>,

    #[serde(skip_serializing_if = "Option::is_none")]
    high_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    low_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_at_time: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_location_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_location_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precipitation_probability: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    relative_humidity: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_feels_like: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weather_report: Option<enums::WeatherReport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wind_direction: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wind_speed: Option<u16>,
}

impl WeatherConditions {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
