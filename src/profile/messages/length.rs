// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for Length {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.start_time = field.one().map(<enums::DateTime>::from),
                3 => msg.total_elapsed_time = field.one().map(<u32>::from),
                4 => msg.total_timer_time = field.one().map(<u32>::from),
                5 => msg.total_strokes = field.one().map(<u16>::from),
                6 => msg.avg_speed = field.one().map(<u16>::from),
                7 => msg.swim_stroke = field.one().map(<enums::SwimStroke>::from),
                9 => msg.avg_swimming_cadence = field.one().map(<u8>::from),
                10 => msg.event_group = field.one().map(<u8>::from),
                11 => msg.total_calories = field.one().map(<u16>::from),
                12 => msg.length_type = field.one().map(<enums::LengthType>::from),
                18 => msg.player_score = field.one().map(<u16>::from),
                19 => msg.opponent_score = field.one().map(<u16>::from),
                20 => msg.stroke_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                21 => msg.zone_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

