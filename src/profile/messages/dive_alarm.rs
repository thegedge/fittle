// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct DiveAlarm {
    message_index: Option<enums::MessageIndex>,
    depth: Option<u32>,
    time: Option<i32>,
    enabled: Option<bool>,
    alarm_type: Option<enums::DiveAlarmType>,
    sound: Option<enums::Tone>,
    dive_types: Option<Vec<enums::SubSport>>,
}

impl DiveAlarm {
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
                0 => msg.depth = content.one().map(<u32>::from),
                1 => msg.time = content.one().map(<i32>::from),
                2 => msg.enabled = content.one().map(<bool>::from),
                3 => msg.alarm_type = content.one().map(<enums::DiveAlarmType>::from),
                4 => msg.sound = content.one().map(<enums::Tone>::from),
                5 => msg.dive_types = content.many().map(|vec| vec.into_iter().map(<enums::SubSport>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

