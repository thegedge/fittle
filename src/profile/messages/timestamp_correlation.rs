// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct TimestampCorrelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_system_timestamp: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<crate::fields::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<u16>,
}

impl TimestampCorrelation {
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
                1 => msg.system_timestamp = content.one().map(<crate::fields::DateTime>::from),
                2 => msg.fractional_system_timestamp = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u16>::from(v)) / 32768.0 - 0.0 })(v))),
                3 => msg.local_timestamp = content.one().map(<crate::fields::LocalDateTime>::from),
                4 => msg.timestamp_ms = content.one().map(<u16>::from),
                5 => msg.system_timestamp_ms = content.one().map(<u16>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
