// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_class: Option<enums::ActivityClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_biking_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_running_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dist_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    elev_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<enums::Gender>,

    #[serde(skip_serializing_if = "Option::is_none")]
    global_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hr_setting: Option<enums::DisplayHeart>,

    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<enums::Language>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_id: Option<enums::UserLocalId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_setting: Option<enums::DisplayPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_setting: Option<enums::DisplayPower>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resting_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sleep_time: Option<enums::LocaltimeIntoDay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_setting: Option<enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_running_step_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_walking_step_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wake_time: Option<enums::LocaltimeIntoDay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_setting: Option<enums::DisplayMeasure>,
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
                1 => msg.gender = content.one().map(<enums::Gender>::from),
                2 => msg.age = content.one().map(<u8>::from),
                3 => msg.height = content.one().map(<u8>::from),
                4 => msg.weight = content.one().map(<u16>::from),
                5 => msg.language = content.one().map(<enums::Language>::from),
                6 => msg.elev_setting = content.one().map(<enums::DisplayMeasure>::from),
                7 => msg.weight_setting = content.one().map(<enums::DisplayMeasure>::from),
                8 => msg.resting_heart_rate = content.one().map(<u8>::from),
                9 => msg.default_max_running_heart_rate = content.one().map(<u8>::from),
                10 => msg.default_max_biking_heart_rate = content.one().map(<u8>::from),
                11 => msg.default_max_heart_rate = content.one().map(<u8>::from),
                12 => msg.hr_setting = content.one().map(<enums::DisplayHeart>::from),
                13 => msg.speed_setting = content.one().map(<enums::DisplayMeasure>::from),
                14 => msg.dist_setting = content.one().map(<enums::DisplayMeasure>::from),
                16 => msg.power_setting = content.one().map(<enums::DisplayPower>::from),
                17 => msg.activity_class = content.one().map(<enums::ActivityClass>::from),
                18 => msg.position_setting = content.one().map(<enums::DisplayPosition>::from),
                21 => msg.temperature_setting = content.one().map(<enums::DisplayMeasure>::from),
                22 => msg.local_id = content.one().map(<enums::UserLocalId>::from),
                23 => msg.global_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                28 => msg.wake_time = content.one().map(<enums::LocaltimeIntoDay>::from),
                29 => msg.sleep_time = content.one().map(<enums::LocaltimeIntoDay>::from),
                30 => msg.height_setting = content.one().map(<enums::DisplayMeasure>::from),
                31 => msg.user_running_step_length = content.one().map(<u16>::from),
                32 => msg.user_walking_step_length = content.one().map(<u16>::from),
                47 => msg.depth_setting = content.one().map(<enums::DisplayMeasure>::from),
                49 => msg.dive_count = content.one().map(<u32>::from),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
