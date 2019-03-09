// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for SdmProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.enabled = field.one().map(<bool>::from),
                1 => msg.sdm_ant_id = field.one().map(<u16>::from),
                2 => msg.sdm_cal_factor = field.one().map(<u16>::from),
                3 => msg.odometer = field.one().map(<u32>::from),
                4 => msg.speed_source = field.one().map(<bool>::from),
                5 => msg.sdm_ant_id_trans_type = field.one().map(<u8>::from),
                7 => msg.odometer_rollover = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

