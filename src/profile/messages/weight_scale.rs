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
pub struct WeightScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_met: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    basal_met: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bone_mass: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    metabolic_age: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    muscle_mass: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    percent_fat: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    percent_hydration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    physique_rating: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visceral_fat_mass: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visceral_fat_rating: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<crate::fields::Mass>,
}

impl WeightScale {
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
                self.weight =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            1 => {
                self.percent_fat =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            2 => {
                self.percent_hydration =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            3 => {
                self.visceral_fat_mass =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.bone_mass =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.muscle_mass =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            7 => {
                self.basal_met =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 4.0 - 0.0 })(value)
                })
            },

            8 => {
                self.physique_rating =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            9 => {
                self.active_met =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 4.0 - 0.0 })(value)
                })
            },

            10 => {
                self.metabolic_age =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Time::new::<uom::si::time::year, u8>)(value)
                })
            },

            11 => {
                self.visceral_fat_rating =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            12 => {
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
