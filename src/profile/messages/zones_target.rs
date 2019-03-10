// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct ZonesTarget {
    max_heart_rate: Option<u8>,
    threshold_heart_rate: Option<u8>,
    functional_threshold_power: Option<u16>,
    hr_calc_type: Option<enums::HrZoneCalc>,
    pwr_calc_type: Option<enums::PwrZoneCalc>,
}

impl ZonesTarget {
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
                1 => msg.max_heart_rate = content.one().map(<u8>::from),
                2 => msg.threshold_heart_rate = content.one().map(<u8>::from),
                3 => msg.functional_threshold_power = content.one().map(<u16>::from),
                5 => msg.hr_calc_type = content.one().map(<enums::HrZoneCalc>::from),
                7 => msg.pwr_calc_type = content.one().map(<enums::PwrZoneCalc>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

