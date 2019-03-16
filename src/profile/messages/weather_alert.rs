// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct WeatherAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    issue_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    report_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<crate::profile::enums::WeatherSeverity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::WeatherSevereType>,
}

impl WeatherAlert {
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
                0 => msg.report_id = content.one().map(<String>::from),
                1 => msg.issue_time = content.one().map(<crate::fields::DateTime>::from),
                2 => msg.expire_time = content.one().map(<crate::fields::DateTime>::from),
                3 => msg.severity = content.one().map(<crate::profile::enums::WeatherSeverity>::from),
                4 => msg.type_ = content.one().map(<crate::profile::enums::WeatherSevereType>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
