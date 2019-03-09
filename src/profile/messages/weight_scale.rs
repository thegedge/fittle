// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct WeightScale {
    timestamp: Option<enums::DateTime>,
    weight: Option<enums::Weight>,
    percent_fat: Option<u16>,
    percent_hydration: Option<u16>,
    visceral_fat_mass: Option<u16>,
    bone_mass: Option<u16>,
    muscle_mass: Option<u16>,
    basal_met: Option<u16>,
    physique_rating: Option<u8>,
    active_met: Option<u16>,
    metabolic_age: Option<u8>,
    visceral_fat_rating: Option<u8>,
    user_profile_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for WeightScale {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.weight = field.one().map(<enums::Weight>::from),
                1 => msg.percent_fat = field.one().map(<u16>::from),
                2 => msg.percent_hydration = field.one().map(<u16>::from),
                3 => msg.visceral_fat_mass = field.one().map(<u16>::from),
                4 => msg.bone_mass = field.one().map(<u16>::from),
                5 => msg.muscle_mass = field.one().map(<u16>::from),
                7 => msg.basal_met = field.one().map(<u16>::from),
                8 => msg.physique_rating = field.one().map(<u8>::from),
                9 => msg.active_met = field.one().map(<u16>::from),
                10 => msg.metabolic_age = field.one().map(<u8>::from),
                11 => msg.visceral_fat_rating = field.one().map(<u8>::from),
                12 => msg.user_profile_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

