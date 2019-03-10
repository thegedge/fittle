// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct HrmProfile {
    message_index: Option<enums::MessageIndex>,
    enabled: Option<bool>,
    hrm_ant_id: Option<u16>,
    log_hrv: Option<bool>,
    hrm_ant_id_trans_type: Option<u8>,
}

impl HrmProfile {
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
                1 => msg.hrm_ant_id = content.one().map(<u16>::from),
                2 => msg.log_hrv = content.one().map(<bool>::from),
                3 => msg.hrm_ant_id_trans_type = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

