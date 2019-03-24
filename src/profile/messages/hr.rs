// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Hr {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp_12: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filtered_bpm: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time256: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl Hr {
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
                0 => msg.fractional_timestamp = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u16>::from(v)) / 32768.0 - 0.0 })(v))),
                1 => msg.time256 = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u8>::from(v)) / 256.0 - 0.0 })(v))),
                6 => msg.filtered_bpm = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.event_timestamp = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1024.0 - 0.0 })(v))).collect()),
                10 => msg.event_timestamp_12 = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, u8>((<u8>::from)(v))).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
