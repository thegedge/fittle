// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_class: Option<crate::profile::enums::ActivityClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_biking_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_running_heart_rate: Option<u8>,

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
    resting_heart_rate: Option<u8>,

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
    weight: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_setting: Option<crate::profile::enums::DisplayMeasure>,
}

impl UserProfile {
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
                0 => msg.friendly_name = content.one().map(<String>::from),
                1 => msg.gender = content.one().map(<crate::profile::enums::Gender>::from),
                2 => msg.age = content.one().map(<u8>::from),
                3 => msg.height = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u8>::from(v)) / 100.0 - 0.0 })(v))),
                4 => msg.weight = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                5 => msg.language = content.one().map(<crate::profile::enums::Language>::from),
                6 => msg.elev_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                7 => msg.weight_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                8 => msg.resting_heart_rate = content.one().map(<u8>::from),
                9 => msg.default_max_running_heart_rate = content.one().map(<u8>::from),
                10 => msg.default_max_biking_heart_rate = content.one().map(<u8>::from),
                11 => msg.default_max_heart_rate = content.one().map(<u8>::from),
                12 => msg.hr_setting = content.one().map(<crate::profile::enums::DisplayHeart>::from),
                13 => msg.speed_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                14 => msg.dist_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                16 => msg.power_setting = content.one().map(<crate::profile::enums::DisplayPower>::from),
                17 => msg.activity_class = content.one().map(<crate::profile::enums::ActivityClass>::from),
                18 => msg.position_setting = content.one().map(<crate::profile::enums::DisplayPosition>::from),
                21 => msg.temperature_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                22 => msg.local_id = content.one().map(<crate::profile::enums::UserLocalId>::from),
                23 => msg.global_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                28 => msg.wake_time = content.one().map(<crate::profile::enums::LocaltimeIntoDay>::from),
                29 => msg.sleep_time = content.one().map(<crate::profile::enums::LocaltimeIntoDay>::from),
                30 => msg.height_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                31 => msg.user_running_step_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                32 => msg.user_walking_step_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                47 => msg.depth_setting = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                49 => msg.dive_count = content.one().map(<u32>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
