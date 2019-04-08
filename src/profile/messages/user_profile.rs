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
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_class: Option<crate::profile::enums::ActivityClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_biking_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_running_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dist_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    elev_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<crate::profile::enums::Gender>,

    #[serde(skip_serializing_if = "Option::is_none")]
    global_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hr_setting: Option<crate::profile::enums::DisplayHeart>,

    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<crate::profile::enums::Language>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_id: Option<crate::profile::enums::UserLocalId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_setting: Option<crate::profile::enums::DisplayPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_setting: Option<crate::profile::enums::DisplayPower>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resting_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sleep_time: Option<crate::profile::enums::LocaltimeIntoDay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_setting: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_running_step_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_walking_step_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wake_time: Option<crate::profile::enums::LocaltimeIntoDay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_setting: Option<crate::profile::enums::DisplayMeasure>,
}

impl UserProfile {
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
                self.friendly_name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.gender =field.one().map(|v| {
                    let value = crate::profile::enums::Gender::from(v);
                    value
                })
            },

            2 => {
                self.age =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Time::new::<uom::si::time::year, u8>)(value)
                })
            },

            3 => {
                self.height =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.weight =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.language =field.one().map(|v| {
                    let value = crate::profile::enums::Language::from(v);
                    value
                })
            },

            6 => {
                self.elev_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            7 => {
                self.weight_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            8 => {
                self.resting_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            9 => {
                self.default_max_running_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            10 => {
                self.default_max_biking_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            11 => {
                self.default_max_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            12 => {
                self.hr_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayHeart::from(v);
                    value
                })
            },

            13 => {
                self.speed_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            14 => {
                self.dist_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            16 => {
                self.power_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayPower::from(v);
                    value
                })
            },

            17 => {
                self.activity_class =field.one().map(|v| {
                    let value = crate::profile::enums::ActivityClass::from(v);
                    value
                })
            },

            18 => {
                self.position_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayPosition::from(v);
                    value
                })
            },

            21 => {
                self.temperature_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            22 => {
                self.local_id =field.one().map(|v| {
                    let value = crate::profile::enums::UserLocalId::from(v);
                    value
                })
            },

            23 => {
                self.global_id =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            28 => {
                self.wake_time =field.one().map(|v| {
                    let value = crate::profile::enums::LocaltimeIntoDay::from(v);
                    value
                })
            },

            29 => {
                self.sleep_time =field.one().map(|v| {
                    let value = crate::profile::enums::LocaltimeIntoDay::from(v);
                    value
                })
            },

            30 => {
                self.height_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            31 => {
                self.user_running_step_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            32 => {
                self.user_walking_step_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            47 => {
                self.depth_setting =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            49 => {
                self.dive_count =field.one().map(|v| {
                    let value = u32::from(v);
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
