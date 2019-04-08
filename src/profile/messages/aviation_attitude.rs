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
pub struct AviationAttitude {
    #[serde(skip_serializing_if = "Option::is_none")]
    accel_lateral: Option<Vec<crate::fields::Acceleration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accel_normal: Option<Vec<crate::fields::Acceleration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attitude_stage_complete: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roll: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<crate::profile::enums::AttitudeStage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_time: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    turn_rate: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    validity: Option<Vec<crate::profile::enums::AttitudeValidity>>,
}

impl AviationAttitude {
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
                self.timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u16>)(value)
                })
            },

            1 => {
                self.system_time =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Time::new::<uom::si::time::millisecond, u32>).collect()
                })
            },

            2 => {
                self.pitch =field.many().map(|v| {
                    let value = v.into_iter().map(i16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10430.0 - 0.0 }).collect()
                })
            },

            3 => {
                self.roll =field.many().map(|v| {
                    let value = v.into_iter().map(i16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10430.0 - 0.0 }).collect()
                })
            },

            4 => {
                self.accel_lateral =field.many().map(|v| {
                    let value = v.into_iter().map(i16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Acceleration::new::<uom::si::acceleration::meter_per_second_squared, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v))).collect()
                })
            },

            5 => {
                self.accel_normal =field.many().map(|v| {
                    let value = v.into_iter().map(i16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Acceleration::new::<uom::si::acceleration::meter_per_second_squared, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v))).collect()
                })
            },

            6 => {
                self.turn_rate =field.many().map(|v| {
                    let value = v.into_iter().map(i16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Frequency::new::<uom::si::frequency::hertz, f64>((|v| { f64::from(v) / 1024.0 - 0.0 })(v))).collect()
                })
            },

            7 => {
                self.stage =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::AttitudeStage::from).collect::<Vec<_>>();
                    value
                })
            },

            8 => {
                self.attitude_stage_complete =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            9 => {
                self.track =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10430.0 - 0.0 }).collect()
                })
            },

            10 => {
                self.validity =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::AttitudeValidity::from).collect::<Vec<_>>();
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
