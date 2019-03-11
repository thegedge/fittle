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
pub struct WeightScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_met: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    basal_met: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bone_mass: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    metabolic_age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    muscle_mass: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    percent_fat: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    percent_hydration: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    physique_rating: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visceral_fat_mass: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visceral_fat_rating: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<enums::Weight>,
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
                0 => msg.weight = content.one().map(<enums::Weight>::from),
                1 => msg.percent_fat = content.one().map(<u16>::from),
                2 => msg.percent_hydration = content.one().map(<u16>::from),
                3 => msg.visceral_fat_mass = content.one().map(<u16>::from),
                4 => msg.bone_mass = content.one().map(<u16>::from),
                5 => msg.muscle_mass = content.one().map(<u16>::from),
                7 => msg.basal_met = content.one().map(<u16>::from),
                8 => msg.physique_rating = content.one().map(<u8>::from),
                9 => msg.active_met = content.one().map(<u16>::from),
                10 => msg.metabolic_age = content.one().map(<u8>::from),
                11 => msg.visceral_fat_rating = content.one().map(<u8>::from),
                12 => msg.user_profile_index = content.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
