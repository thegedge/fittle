// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

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
                0 => msg.weight = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                1 => msg.percent_fat = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                2 => msg.percent_hydration = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                3 => msg.visceral_fat_mass = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                4 => msg.bone_mass = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                5 => msg.muscle_mass = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                7 => msg.basal_met = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 4.0 - 0.0 }),
                8 => msg.physique_rating = content.one().map(<u8>::from),
                9 => msg.active_met = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 4.0 - 0.0 }),
                10 => msg.metabolic_age = content.one().map(|v| crate::fields::Time::new::<uom::si::time::year, u8>((<u8>::from)(v))),
                11 => msg.visceral_fat_rating = content.one().map(<u8>::from),
                12 => msg.user_profile_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
