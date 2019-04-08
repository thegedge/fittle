// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

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
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.weather_report =field.one().map(|v| {
                    let value = crate::profile::enums::WeatherReport::from(v);
                    value
                })
            },

            1 => {
                self.temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            2 => {
                self.condition =field.one().map(|v| {
                    let value = crate::profile::enums::WeatherStatus::from(v);
                    value
                })
            },

            3 => {
                self.wind_direction =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            4 => {
                self.wind_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.precipitation_probability =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            6 => {
                self.temperature_feels_like =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            7 => {
                self.relative_humidity =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            8 => {
                self.location =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            9 => {
                self.observed_at_time =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            10 => {
                self.observed_location_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            11 => {
                self.observed_location_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            12 => {
                self.day_of_week =field.one().map(|v| {
                    let value = crate::profile::enums::DayOfWeek::from(v);
                    value
                })
            },

            13 => {
                self.high_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            14 => {
                self.low_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
