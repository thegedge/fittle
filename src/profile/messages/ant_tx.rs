// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct AntTx {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    mesg_id: Option<u8>,
    mesg_data: Option<Vec<u8>>,
    channel_number: Option<u8>,
    data: Option<Vec<u8>>,
}

impl From<Vec<(u8, Field)>> for AntTx {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.mesg_id = field.one().map(<u8>::from),
                2 => msg.mesg_data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                3 => msg.channel_number = field.one().map(<u8>::from),
                4 => msg.data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

