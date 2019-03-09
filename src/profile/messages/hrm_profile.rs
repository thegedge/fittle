// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct HrmProfile {
    message_index: Option<enums::MessageIndex>,
    enabled: Option<bool>,
    hrm_ant_id: Option<u16>,
    log_hrv: Option<bool>,
    hrm_ant_id_trans_type: Option<u8>,
}

impl From<Vec<(u8, Field)>> for HrmProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.enabled = field.one().map(<bool>::from),
                1 => msg.hrm_ant_id = field.one().map(<u16>::from),
                2 => msg.log_hrv = field.one().map(<bool>::from),
                3 => msg.hrm_ant_id_trans_type = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

