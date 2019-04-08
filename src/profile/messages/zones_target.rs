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
            1 => {
                self.max_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.threshold_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            3 => {
                self.functional_threshold_power =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            5 => {
                self.hr_calc_type =field.one().map(|v| {
                    let value = crate::profile::enums::HrZoneCalc::from(v);
                    value
                })
            },

            7 => {
                self.pwr_calc_type =field.one().map(|v| {
                    let value = crate::profile::enums::PwrZoneCalc::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
