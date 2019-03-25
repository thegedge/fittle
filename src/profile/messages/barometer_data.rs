// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct BarometerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    baro_pres: Option<Vec<crate::fields::Pressure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sample_time_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,
}

impl BarometerData {
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
                0 => msg.timestamp_ms = content.one().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u16>((<u16>::from)(v))),
                1 => msg.sample_time_offset = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, u16>((<u16>::from)(v))).collect()),
                2 => msg.baro_pres = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Pressure::new::<uom::si::pressure::pascal, u32>((<u32>::from)(v))).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
