// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct AntChannelId {
    channel_number: Option<u8>,
    device_type: Option<u8>,
    device_number: Option<u16>,
    transmission_type: Option<u8>,
    device_index: Option<enums::DeviceIndex>,
}

impl From<Vec<(u8, Field)>> for AntChannelId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.channel_number = field.one().map(<u8>::from),
                1 => msg.device_type = field.one().map(<u8>::from),
                2 => msg.device_number = field.one().map(<u16>::from),
                3 => msg.transmission_type = field.one().map(<u8>::from),
                4 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

