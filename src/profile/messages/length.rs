// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

#[derive(Debug, Default, Serialize)]
pub struct Length {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_swimming_cadence: Option<crate::fields::Frequency>,

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
    total_calories: Option<crate::fields::Energy>,

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
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.event =field.one().map(|v| {
                    let value = crate::profile::enums::Event::from(v);
                    value
                })
            },

            1 => {
                self.event_type =field.one().map(|v| {
                    let value = crate::profile::enums::EventType::from(v);
                    value
                })
            },

            2 => {
                self.start_time =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            3 => {
                self.total_elapsed_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.total_timer_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.total_strokes =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            6 => {
                self.avg_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            7 => {
                self.swim_stroke =field.one().map(|v| {
                    let value = crate::profile::enums::SwimStroke::from(v);
                    value
                })
            },

            9 => {
                self.avg_swimming_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            10 => {
                self.event_group =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            11 => {
                self.total_calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            12 => {
                self.length_type =field.one().map(|v| {
                    let value = crate::profile::enums::LengthType::from(v);
                    value
                })
            },

            18 => {
                self.player_score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            19 => {
                self.opponent_score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            20 => {
                self.stroke_count =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            21 => {
                self.zone_count =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            254 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
