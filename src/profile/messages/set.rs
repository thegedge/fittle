// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Set {
    timestamp: Option<enums::DateTime>,
    duration: Option<u32>,
    repetitions: Option<u16>,
    weight: Option<u16>,
    set_type: Option<enums::SetType>,
    start_time: Option<enums::DateTime>,
    category: Option<Vec<enums::ExerciseCategory>>,
    category_subtype: Option<Vec<u16>>,
    weight_display_unit: Option<enums::FitBaseUnit>,
    message_index: Option<enums::MessageIndex>,
    wkt_step_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for Set {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.duration = field.one().map(<u32>::from),
                3 => msg.repetitions = field.one().map(<u16>::from),
                4 => msg.weight = field.one().map(<u16>::from),
                5 => msg.set_type = field.one().map(<enums::SetType>::from),
                6 => msg.start_time = field.one().map(<enums::DateTime>::from),
                7 => msg.category = field.many().map(|vec| vec.into_iter().map(<enums::ExerciseCategory>::from).collect()),
                8 => msg.category_subtype = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                9 => msg.weight_display_unit = field.one().map(<enums::FitBaseUnit>::from),
                10 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                11 => msg.wkt_step_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

