// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct SdmProfile {
    message_index: Option<enums::MessageIndex>,
    enabled: Option<bool>,
    sdm_ant_id: Option<u16>,
    sdm_cal_factor: Option<u16>,
    odometer: Option<u32>,
    speed_source: Option<bool>,
    sdm_ant_id_trans_type: Option<u8>,
    odometer_rollover: Option<u8>,
}

impl SdmProfile {
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
                0 => msg.enabled = content.one().map(<bool>::from),
                1 => msg.sdm_ant_id = content.one().map(<u16>::from),
                2 => msg.sdm_cal_factor = content.one().map(<u16>::from),
                3 => msg.odometer = content.one().map(<u32>::from),
                4 => msg.speed_source = content.one().map(<bool>::from),
                5 => msg.sdm_ant_id_trans_type = content.one().map(<u8>::from),
                7 => msg.odometer_rollover = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

