// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Length {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_swimming_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    length_type: Option<crate::profile::enums::LengthType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    player_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_count: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swim_stroke: Option<crate::profile::enums::SwimStroke>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_elapsed_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_strokes: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_timer_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone_count: Option<Vec<u16>>,
}

impl Length {
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
                0 => msg.event = content.one().map(<crate::profile::enums::Event>::from),
                1 => msg.event_type = content.one().map(<crate::profile::enums::EventType>::from),
                2 => msg.start_time = content.one().map(<crate::fields::DateTime>::from),
                3 => msg.total_elapsed_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                4 => msg.total_timer_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                5 => msg.total_strokes = content.one().map(<u16>::from),
                6 => msg.avg_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                7 => msg.swim_stroke = content.one().map(<crate::profile::enums::SwimStroke>::from),
                9 => msg.avg_swimming_cadence = content.one().map(<u8>::from),
                10 => msg.event_group = content.one().map(<u8>::from),
                11 => msg.total_calories = content.one().map(<u16>::from),
                12 => msg.length_type = content.one().map(<crate::profile::enums::LengthType>::from),
                18 => msg.player_score = content.one().map(<u16>::from),
                19 => msg.opponent_score = content.one().map(<u16>::from),
                20 => msg.stroke_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                21 => msg.zone_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
