// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct DiveSummary {
    timestamp: Option<enums::DateTime>,
    reference_mesg: Option<enums::MesgNum>,
    reference_index: Option<enums::MessageIndex>,
    avg_depth: Option<u32>,
    max_depth: Option<u32>,
    surface_interval: Option<u32>,
    start_cns: Option<u8>,
    end_cns: Option<u8>,
    start_n2: Option<u16>,
    end_n2: Option<u16>,
    o2_toxicity: Option<u16>,
    dive_number: Option<u32>,
    bottom_time: Option<u32>,
}

impl DiveSummary {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.reference_mesg = content.one().map(<enums::MesgNum>::from),
                1 => msg.reference_index = content.one().map(<enums::MessageIndex>::from),
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
                _ => (),
            };
        }
        Ok(msg)
    }
}

