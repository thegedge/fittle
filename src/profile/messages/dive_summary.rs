// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct DiveSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_depth: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_cns: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_n2: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    o2_toxicity: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_mesg: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_cns: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_n2: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    surface_interval: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl DiveSummary {
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
                0 => msg.reference_mesg = content.one().map(<crate::profile::enums::MesgNum>::from),
                1 => msg.reference_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                2 => msg.avg_depth = content.one().map(<u32>::from),
                3 => msg.max_depth = content.one().map(<u32>::from),
                4 => msg.surface_interval = content.one().map(<u32>::from),
                5 => msg.start_cns = content.one().map(<u8>::from),
                6 => msg.end_cns = content.one().map(<u8>::from),
                7 => msg.start_n2 = content.one().map(<u16>::from),
                8 => msg.end_n2 = content.one().map(<u16>::from),
                9 => msg.o2_toxicity = content.one().map(<u16>::from),
                10 => msg.dive_number = content.one().map(<u32>::from),
                11 => msg.bottom_time = content.one().map(<u32>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
