// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

#[derive(Debug, Default, Serialize)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    connectivity_supported: Option<crate::profile::enums::ConnectivityCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    workouts_supported: Option<crate::profile::enums::WorkoutCapabilities>,
}

impl Capabilities {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            21 => {
                self.workouts_supported =field.one().map(|v| {
                    let value = crate::profile::enums::WorkoutCapabilities::from(v);
                    value
                })
            },

            23 => {
                self.connectivity_supported =field.one().map(|v| {
                    let value = crate::profile::enums::ConnectivityCapabilities::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
