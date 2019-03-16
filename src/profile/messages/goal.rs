// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Goal {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<crate::profile::enums::GoalRecurrence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence_value: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<crate::profile::enums::GoalSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::Goal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
}

impl Goal {
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
                0 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                1 => msg.sub_sport = content.one().map(<crate::profile::enums::SubSport>::from),
                2 => msg.start_date = content.one().map(<crate::fields::DateTime>::from),
                3 => msg.end_date = content.one().map(<crate::fields::DateTime>::from),
                4 => msg.type_ = content.one().map(<crate::profile::enums::Goal>::from),
                5 => msg.value = content.one().map(<u32>::from),
                6 => msg.repeat = content.one().map(<bool>::from),
                7 => msg.target_value = content.one().map(<u32>::from),
                8 => msg.recurrence = content.one().map(<crate::profile::enums::GoalRecurrence>::from),
                9 => msg.recurrence_value = content.one().map(<u16>::from),
                10 => msg.enabled = content.one().map(<bool>::from),
                11 => msg.source = content.one().map(<crate::profile::enums::GoalSource>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
