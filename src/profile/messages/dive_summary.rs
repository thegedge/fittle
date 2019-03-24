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
    avg_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_cns: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_n2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    o2_toxicity: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_mesg: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_cns: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_n2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    surface_interval: Option<crate::fields::Time>,

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
                2 => msg.avg_depth = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                3 => msg.max_depth = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                4 => msg.surface_interval = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1.0 - 0.0 })(v))),
                5 => msg.start_cns = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 1.0 - 0.0 }),
                6 => msg.end_cns = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 1.0 - 0.0 }),
                7 => msg.start_n2 = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1.0 - 0.0 }),
                8 => msg.end_n2 = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1.0 - 0.0 }),
                9 => msg.o2_toxicity = content.one().map(<u16>::from),
                10 => msg.dive_number = content.one().map(<u32>::from),
                11 => msg.bottom_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
