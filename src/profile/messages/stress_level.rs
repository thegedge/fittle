// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct StressLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    stress_level_value: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stress_level_time: Option<enums::DateTime>,

}

impl StressLevel {
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
                0 => msg.stress_level_value = content.one().map(<i16>::from),
                1 => msg.stress_level_time = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

