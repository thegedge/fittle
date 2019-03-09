// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct MesgCapabilities {
    message_index: Option<enums::MessageIndex>,
    file: Option<enums::File>,
    mesg_num: Option<enums::MesgNum>,
    count_type: Option<enums::MesgCount>,
    count: Option<u16>,
}

impl From<Vec<(u8, Field)>> for MesgCapabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.file = field.one().map(<enums::File>::from),
                1 => msg.mesg_num = field.one().map(<enums::MesgNum>::from),
                2 => msg.count_type = field.one().map(<enums::MesgCount>::from),
                3 => msg.count = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

