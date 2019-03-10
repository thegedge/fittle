// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct WeatherAlert {
    timestamp: Option<enums::DateTime>,
    report_id: Option<String>,
    issue_time: Option<enums::DateTime>,
    expire_time: Option<enums::DateTime>,
    severity: Option<enums::WeatherSeverity>,
    type_: Option<enums::WeatherSevereType>,
}

impl WeatherAlert {
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
                0 => msg.report_id = content.one().map(<String>::from),
                1 => msg.issue_time = content.one().map(<enums::DateTime>::from),
                2 => msg.expire_time = content.one().map(<enums::DateTime>::from),
                3 => msg.severity = content.one().map(<enums::WeatherSeverity>::from),
                4 => msg.type_ = content.one().map(<enums::WeatherSevereType>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

