// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct BloodPressure {
    #[serde(skip_serializing_if = "Option::is_none")]
    diastolic_pressure: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate_type: Option<crate::profile::enums::HrType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_3_sample_mean: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_evening_values: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map_morning_values: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mean_arterial_pressure: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<crate::profile::enums::BpStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    systolic_pressure: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_profile_index: Option<crate::profile::enums::MessageIndex>,
}

impl BloodPressure {
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
                0 => msg.systolic_pressure = content.one().map(<u16>::from),
                1 => msg.diastolic_pressure = content.one().map(<u16>::from),
                2 => msg.mean_arterial_pressure = content.one().map(<u16>::from),
                3 => msg.map_3_sample_mean = content.one().map(<u16>::from),
                4 => msg.map_morning_values = content.one().map(<u16>::from),
                5 => msg.map_evening_values = content.one().map(<u16>::from),
                6 => msg.heart_rate = content.one().map(<u8>::from),
                7 => msg.heart_rate_type = content.one().map(<crate::profile::enums::HrType>::from),
                8 => msg.status = content.one().map(<crate::profile::enums::BpStatus>::from),
                9 => msg.user_profile_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
