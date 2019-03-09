// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct DeveloperDataId {
    developer_id: Option<Vec<u8>>,
    application_id: Option<Vec<u8>>,
    manufacturer_id: Option<enums::Manufacturer>,
    developer_data_index: Option<u8>,
    application_version: Option<u32>,
}

impl From<Vec<(u8, Field)>> for DeveloperDataId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.developer_id = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.application_id = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                2 => msg.manufacturer_id = field.one().map(<enums::Manufacturer>::from),
                3 => msg.developer_data_index = field.one().map(<u8>::from),
                4 => msg.application_version = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

