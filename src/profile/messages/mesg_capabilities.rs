// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct MesgCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    count_type: Option<crate::profile::enums::MesgCount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<crate::profile::enums::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_num: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,
}

impl MesgCapabilities {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.file = content.one().map(<crate::profile::enums::File>::from),
                1 => msg.mesg_num = content.one().map(<crate::profile::enums::MesgNum>::from),
                2 => msg.count_type = content.one().map(<crate::profile::enums::MesgCount>::from),
                3 => msg.count = content.one().map(<u16>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
