// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct MonitoringInfo {
    timestamp: Option<enums::DateTime>,
    local_timestamp: Option<enums::LocalDateTime>,
    activity_type: Option<Vec<enums::ActivityType>>,
    cycles_to_distance: Option<Vec<u16>>,
    cycles_to_calories: Option<Vec<u16>>,
    resting_metabolic_rate: Option<u16>,
}

impl MonitoringInfo {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.local_timestamp = content.one().map(<enums::LocalDateTime>::from),
                1 => msg.activity_type = content.many().map(|vec| vec.into_iter().map(<enums::ActivityType>::from).collect()),
                3 => msg.cycles_to_distance = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.cycles_to_calories = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.resting_metabolic_rate = content.one().map(<u16>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

