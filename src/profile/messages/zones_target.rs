// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct ZonesTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    functional_threshold_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hr_calc_type: Option<crate::profile::enums::HrZoneCalc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pwr_calc_type: Option<crate::profile::enums::PwrZoneCalc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_heart_rate: Option<u8>,
}

impl ZonesTarget {
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
                1 => msg.max_heart_rate = content.one().map(<u8>::from),
                2 => msg.threshold_heart_rate = content.one().map(<u8>::from),
                3 => msg.functional_threshold_power = content.one().map(<u16>::from),
                5 => msg.hr_calc_type = content.one().map(<crate::profile::enums::HrZoneCalc>::from),
                7 => msg.pwr_calc_type = content.one().map(<crate::profile::enums::PwrZoneCalc>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
