// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct DiveGas {
    message_index: Option<enums::MessageIndex>,
    helium_content: Option<u8>,
    oxygen_content: Option<u8>,
    status: Option<enums::DiveGasStatus>,
}

impl From<Vec<(u8, Field)>> for DiveGas {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.helium_content = field.one().map(<u8>::from),
                1 => msg.oxygen_content = field.one().map(<u8>::from),
                2 => msg.status = field.one().map(<enums::DiveGasStatus>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

