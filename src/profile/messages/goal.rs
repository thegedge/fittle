// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Goal {
    message_index: Option<enums::MessageIndex>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    start_date: Option<enums::DateTime>,
    end_date: Option<enums::DateTime>,
    type_: Option<enums::Goal>,
    value: Option<u32>,
    repeat: Option<bool>,
    target_value: Option<u32>,
    recurrence: Option<enums::GoalRecurrence>,
    recurrence_value: Option<u16>,
    enabled: Option<bool>,
    source: Option<enums::GoalSource>,
}

impl From<Vec<(u8, Field)>> for Goal {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                2 => msg.start_date = field.one().map(<enums::DateTime>::from),
                3 => msg.end_date = field.one().map(<enums::DateTime>::from),
                4 => msg.type_ = field.one().map(<enums::Goal>::from),
                5 => msg.value = field.one().map(<u32>::from),
                6 => msg.repeat = field.one().map(<bool>::from),
                7 => msg.target_value = field.one().map(<u32>::from),
                8 => msg.recurrence = field.one().map(<enums::GoalRecurrence>::from),
                9 => msg.recurrence_value = field.one().map(<u16>::from),
                10 => msg.enabled = field.one().map(<bool>::from),
                11 => msg.source = field.one().map(<enums::GoalSource>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

