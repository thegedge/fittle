// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Workout {
    sport: Option<enums::Sport>,
    capabilities: Option<enums::WorkoutCapabilities>,
    num_valid_steps: Option<u16>,
    wkt_name: Option<String>,
    sub_sport: Option<enums::SubSport>,
    pool_length: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
}

impl Workout {
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
                4 => msg.sport = content.one().map(<enums::Sport>::from),
                5 => msg.capabilities = content.one().map(<enums::WorkoutCapabilities>::from),
                6 => msg.num_valid_steps = content.one().map(<u16>::from),
                8 => msg.wkt_name = content.one().map(<String>::from),
                11 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                14 => msg.pool_length = content.one().map(<u16>::from),
                15 => msg.pool_length_unit = content.one().map(<enums::DisplayMeasure>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

