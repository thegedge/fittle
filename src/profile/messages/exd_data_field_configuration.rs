// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ExdDataFieldConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_count: Option<u8>,
    display_type: Option<enums::ExdDisplayType>,
    title: Option<Vec<String>>,
}

impl From<Vec<(u8, Field)>> for ExdDataFieldConfiguration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.screen_index = field.one().map(<u8>::from),
                1 => msg.concept_field = field.one().map(<u8>::from),
                2 => msg.field_id = field.one().map(<u8>::from),
                3 => msg.concept_count = field.one().map(<u8>::from),
                4 => msg.display_type = field.one().map(<enums::ExdDisplayType>::from),
                5 => msg.title = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

