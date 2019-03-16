// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity_supported: Option<crate::profile::enums::ConnectivityCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    workouts_supported: Option<crate::profile::enums::WorkoutCapabilities>,
}

impl Capabilities {
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
                21 => msg.workouts_supported = content.one().map(<crate::profile::enums::WorkoutCapabilities>::from),
                23 => msg.connectivity_supported = content.one().map(<crate::profile::enums::ConnectivityCapabilities>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
