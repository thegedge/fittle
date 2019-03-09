// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ExdDataConceptConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_index: Option<u8>,
    data_page: Option<u8>,
    concept_key: Option<u8>,
    scaling: Option<u8>,
    data_units: Option<enums::ExdDataUnits>,
    qualifier: Option<enums::ExdQualifiers>,
    descriptor: Option<enums::ExdDescriptors>,
    is_signed: Option<bool>,
}

impl From<Vec<(u8, Field)>> for ExdDataConceptConfiguration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.screen_index = field.one().map(<u8>::from),
                1 => msg.concept_field = field.one().map(<u8>::from),
                2 => msg.field_id = field.one().map(<u8>::from),
                3 => msg.concept_index = field.one().map(<u8>::from),
                4 => msg.data_page = field.one().map(<u8>::from),
                5 => msg.concept_key = field.one().map(<u8>::from),
                6 => msg.scaling = field.one().map(<u8>::from),
                8 => msg.data_units = field.one().map(<enums::ExdDataUnits>::from),
                9 => msg.qualifier = field.one().map(<enums::ExdQualifiers>::from),
                10 => msg.descriptor = field.one().map(<enums::ExdDescriptors>::from),
                11 => msg.is_signed = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

