// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct MonitoringInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<Vec<crate::profile::enums::ActivityType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_to_calories: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_to_distance: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<crate::fields::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resting_metabolic_rate: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl MonitoringInfo {
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
                0 => msg.local_timestamp = content.one().map(<crate::fields::LocalDateTime>::from),
                1 => msg.activity_type = content.many().map(|vec| vec.into_iter().map(<crate::profile::enums::ActivityType>::from).collect()),
                3 => msg.cycles_to_distance = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 5000.0 - 0.0 }).collect()),
                4 => msg.cycles_to_calories = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 5000.0 - 0.0 }).collect()),
                5 => msg.resting_metabolic_rate = content.one().map(<u16>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
