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
pub struct OneDSensorCalibration {
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration_divisor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibration_factor: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    level_shift: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset_cal: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sensor_type: Option<crate::profile::enums::SensorType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl OneDSensorCalibration {
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
                self.sensor_type =field.one().map(|v| {
                    let value = crate::profile::enums::SensorType::from(v);
                    value
                })
            },

            1 => {
                self.calibration_factor =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            2 => {
                self.calibration_divisor =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            3 => {
                self.level_shift =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            4 => {
                self.offset_cal =field.one().map(|v| {
                    let value = i32::from(v);
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
