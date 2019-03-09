// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct ExerciseTitle {
    message_index: Option<enums::MessageIndex>,
    exercise_category: Option<enums::ExerciseCategory>,
    exercise_name: Option<u16>,
    wkt_step_name: Option<Vec<String>>,
}

impl From<Vec<(u8, Field)>> for ExerciseTitle {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.exercise_category = field.one().map(<enums::ExerciseCategory>::from),
                1 => msg.exercise_name = field.one().map(<u16>::from),
                2 => msg.wkt_step_name = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

