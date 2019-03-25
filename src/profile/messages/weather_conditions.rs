// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct WeatherConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<crate::profile::enums::WeatherStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<crate::profile::enums::DayOfWeek>,

    #[serde(skip_serializing_if = "Option::is_none")]
    high_temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    low_temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_at_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_location_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observed_location_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precipitation_probability: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    relative_humidity: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_feels_like: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weather_report: Option<crate::profile::enums::WeatherReport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wind_direction: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wind_speed: Option<crate::fields::Velocity>,
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
                0 => msg.weather_report = content.one().map(<crate::profile::enums::WeatherReport>::from),
                1 => msg.temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                2 => msg.condition = content.one().map(<crate::profile::enums::WeatherStatus>::from),
                3 => msg.wind_direction = content.one().map(<u16>::from),
                4 => msg.wind_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                5 => msg.precipitation_probability = content.one().map(<u8>::from),
                6 => msg.temperature_feels_like = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                7 => msg.relative_humidity = content.one().map(<u8>::from),
                8 => msg.location = content.one().map(<String>::from),
                9 => msg.observed_at_time = content.one().map(<crate::fields::DateTime>::from),
                10 => msg.observed_location_lat = content.one().map(<i32>::from),
                11 => msg.observed_location_long = content.one().map(<i32>::from),
                12 => msg.day_of_week = content.one().map(<crate::profile::enums::DayOfWeek>::from),
                13 => msg.high_temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                14 => msg.low_temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
