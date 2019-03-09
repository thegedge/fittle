// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct MemoGlob {
    part_index: Option<u32>,
    memo: Option<Vec<u8>>,
    message_number: Option<u16>,
    message_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for MemoGlob {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                250 => msg.part_index = field.one().map(<u32>::from),
                0 => msg.memo = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.message_number = field.one().map(<u16>::from),
                2 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

