// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct WorkoutSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_valid_steps: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    first_step_index: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length_unit: Option<enums::DisplayMeasure>,

}

impl WorkoutSession {
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
                2 => msg.num_valid_steps = content.one().map(<u16>::from),
                3 => msg.first_step_index = content.one().map(<u16>::from),
                4 => msg.pool_length = content.one().map(<u16>::from),
                5 => msg.pool_length_unit = content.one().map(<enums::DisplayMeasure>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

