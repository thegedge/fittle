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
pub struct MagnetometerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_x: Option<Vec<crate::fields::MagneticFluxDensity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_y: Option<Vec<crate::fields::MagneticFluxDensity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calibrated_mag_z: Option<Vec<crate::fields::MagneticFluxDensity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_x: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_y: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mag_z: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sample_time_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,
}

impl MagnetometerData {
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
                self.timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u16>)(value)
                })
            },

            1 => {
                self.sample_time_offset =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Time::new::<uom::si::time::millisecond, u16>).collect()
                })
            },

            2 => {
                self.mag_x =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            3 => {
                self.mag_y =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            4 => {
                self.mag_z =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            5 => {
                self.calibrated_mag_x =field.many().map(|v| {
                    let value = v.into_iter().map(f32::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::gauss, f32>).collect()
                })
            },

            6 => {
                self.calibrated_mag_y =field.many().map(|v| {
                    let value = v.into_iter().map(f32::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::gauss, f32>).collect()
                })
            },

            7 => {
                self.calibrated_mag_z =field.many().map(|v| {
                    let value = v.into_iter().map(f32::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::gauss, f32>).collect()
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
