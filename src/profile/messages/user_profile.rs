// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct UserProfile {
    message_index: Option<enums::MessageIndex>,
    friendly_name: Option<String>,
    gender: Option<enums::Gender>,
    age: Option<u8>,
    height: Option<u8>,
    weight: Option<u16>,
    language: Option<enums::Language>,
    elev_setting: Option<enums::DisplayMeasure>,
    weight_setting: Option<enums::DisplayMeasure>,
    resting_heart_rate: Option<u8>,
    default_max_running_heart_rate: Option<u8>,
    default_max_biking_heart_rate: Option<u8>,
    default_max_heart_rate: Option<u8>,
    hr_setting: Option<enums::DisplayHeart>,
    speed_setting: Option<enums::DisplayMeasure>,
    dist_setting: Option<enums::DisplayMeasure>,
    power_setting: Option<enums::DisplayPower>,
    activity_class: Option<enums::ActivityClass>,
    position_setting: Option<enums::DisplayPosition>,
    temperature_setting: Option<enums::DisplayMeasure>,
    local_id: Option<enums::UserLocalId>,
    global_id: Option<Vec<u8>>,
    wake_time: Option<enums::LocaltimeIntoDay>,
    sleep_time: Option<enums::LocaltimeIntoDay>,
    height_setting: Option<enums::DisplayMeasure>,
    user_running_step_length: Option<u16>,
    user_walking_step_length: Option<u16>,
    depth_setting: Option<enums::DisplayMeasure>,
    dive_count: Option<u32>,
}

impl UserProfile {
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
                _ => (),
            };
        }
        Ok(msg)
    }
}
