// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Workout {
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<crate::profile::enums::WorkoutCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_valid_steps: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length_unit: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_name: Option<String>,
}

impl Workout {
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
                4 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                5 => msg.capabilities = content.one().map(<crate::profile::enums::WorkoutCapabilities>::from),
                6 => msg.num_valid_steps = content.one().map(<u16>::from),
                8 => msg.wkt_name = content.one().map(<String>::from),
                11 => msg.sub_sport = content.one().map(<crate::profile::enums::SubSport>::from),
                14 => msg.pool_length = content.one().map(<u16>::from),
                15 => msg.pool_length_unit = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
