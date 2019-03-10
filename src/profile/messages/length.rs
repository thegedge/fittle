// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Length {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    start_time: Option<enums::DateTime>,
    total_elapsed_time: Option<u32>,
    total_timer_time: Option<u32>,
    total_strokes: Option<u16>,
    avg_speed: Option<u16>,
    swim_stroke: Option<enums::SwimStroke>,
    avg_swimming_cadence: Option<u8>,
    event_group: Option<u8>,
    total_calories: Option<u16>,
    length_type: Option<enums::LengthType>,
    player_score: Option<u16>,
    opponent_score: Option<u16>,
    stroke_count: Option<Vec<u16>>,
    zone_count: Option<Vec<u16>>,
}

impl Length {
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
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.event = content.one().map(<enums::Event>::from),
                1 => msg.event_type = content.one().map(<enums::EventType>::from),
                2 => msg.start_time = content.one().map(<enums::DateTime>::from),
                3 => msg.total_elapsed_time = content.one().map(<u32>::from),
                4 => msg.total_timer_time = content.one().map(<u32>::from),
                5 => msg.total_strokes = content.one().map(<u16>::from),
                6 => msg.avg_speed = content.one().map(<u16>::from),
                7 => msg.swim_stroke = content.one().map(<enums::SwimStroke>::from),
                9 => msg.avg_swimming_cadence = content.one().map(<u8>::from),
                10 => msg.event_group = content.one().map(<u8>::from),
                11 => msg.total_calories = content.one().map(<u16>::from),
                12 => msg.length_type = content.one().map(<enums::LengthType>::from),
                18 => msg.player_score = content.one().map(<u16>::from),
                19 => msg.opponent_score = content.one().map(<u16>::from),
                20 => msg.stroke_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                21 => msg.zone_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

