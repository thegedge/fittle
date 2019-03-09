// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct FileCapabilities {
    message_index: Option<enums::MessageIndex>,
    type_: Option<enums::File>,
    flags: Option<enums::FileFlags>,
    directory: Option<String>,
    max_count: Option<u16>,
    max_size: Option<u32>,
}

impl From<Vec<(u8, Field)>> for FileCapabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.type_ = field.one().map(<enums::File>::from),
                1 => msg.flags = field.one().map(<enums::FileFlags>::from),
                2 => msg.directory = field.one().map(<String>::from),
                3 => msg.max_count = field.one().map(<u16>::from),
                4 => msg.max_size = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

