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
pub struct DiveAlarm {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_type: Option<enums::DiveAlarmType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_types: Option<Vec<enums::SubSport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<enums::Tone>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

impl DiveAlarm {
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
                0 => msg.depth = content.one().map(<u32>::from),
                1 => msg.time = content.one().map(<i32>::from),
                2 => msg.enabled = content.one().map(<bool>::from),
                3 => msg.alarm_type = content.one().map(<enums::DiveAlarmType>::from),
                4 => msg.sound = content.one().map(<enums::Tone>::from),
                5 => msg.dive_types = content.many().map(|vec| vec.into_iter().map(<enums::SubSport>::from).collect()),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
