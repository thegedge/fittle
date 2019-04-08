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
pub struct BloodPressure {
    #[serde(skip_serializing_if = "Option::is_none")]
    diastolic_pressure: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate_type: Option<crate::profile::enums::HrType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_3_sample_mean: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_evening_values: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_morning_values: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mean_arterial_pressure: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<crate::profile::enums::BpStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    systolic_pressure: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_index: Option<crate::profile::enums::MessageIndex>,
}

impl BloodPressure {
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
                self.systolic_pressure =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            1 => {
                self.diastolic_pressure =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            2 => {
                self.mean_arterial_pressure =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            3 => {
                self.map_3_sample_mean =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            4 => {
                self.map_morning_values =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            5 => {
                self.map_evening_values =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::millimeter_of_mercury, u16>)(value)
                })
            },

            6 => {
                self.heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            7 => {
                self.heart_rate_type =field.one().map(|v| {
                    let value = crate::profile::enums::HrType::from(v);
                    value
                })
            },

            8 => {
                self.status =field.one().map(|v| {
                    let value = crate::profile::enums::BpStatus::from(v);
                    value
                })
            },

            9 => {
                self.user_profile_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
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
