// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl Goal {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                0 => msg.sport = content.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                2 => msg.start_date = content.one().map(<enums::DateTime>::from),
                3 => msg.end_date = content.one().map(<enums::DateTime>::from),
                4 => msg.type_ = content.one().map(<enums::Goal>::from),
                5 => msg.value = content.one().map(<u32>::from),
                6 => msg.repeat = content.one().map(<bool>::from),
                7 => msg.target_value = content.one().map(<u32>::from),
                8 => msg.recurrence = content.one().map(<enums::GoalRecurrence>::from),
                9 => msg.recurrence_value = content.one().map(<u16>::from),
                10 => msg.enabled = content.one().map(<bool>::from),
                11 => msg.source = content.one().map(<enums::GoalSource>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

