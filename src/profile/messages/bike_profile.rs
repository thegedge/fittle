// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct BikeProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spd_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_cad_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spdcad_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_power_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_wheelsize: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_wheelsize: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_cal_factor: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_wheel_cal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_power_zero: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spd_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cad_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spdcad_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    crank_length: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spd_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_cad_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spdcad_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_power_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer_rollover: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shimano_di2_enabled: Option<bool>,

}

impl BikeProfile {
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
                0 => msg.name = content.one().map(<String>::from),
                1 => msg.sport = content.one().map(<enums::Sport>::from),
                2 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                3 => msg.odometer = content.one().map(<u32>::from),
                4 => msg.bike_spd_ant_id = content.one().map(<u16>::from),
                5 => msg.bike_cad_ant_id = content.one().map(<u16>::from),
                6 => msg.bike_spdcad_ant_id = content.one().map(<u16>::from),
                7 => msg.bike_power_ant_id = content.one().map(<u16>::from),
                8 => msg.custom_wheelsize = content.one().map(<u16>::from),
                9 => msg.auto_wheelsize = content.one().map(<u16>::from),
                10 => msg.bike_weight = content.one().map(<u16>::from),
                11 => msg.power_cal_factor = content.one().map(<u16>::from),
                12 => msg.auto_wheel_cal = content.one().map(<bool>::from),
                13 => msg.auto_power_zero = content.one().map(<bool>::from),
                14 => msg.id = content.one().map(<u8>::from),
                15 => msg.spd_enabled = content.one().map(<bool>::from),
                16 => msg.cad_enabled = content.one().map(<bool>::from),
                17 => msg.spdcad_enabled = content.one().map(<bool>::from),
                18 => msg.power_enabled = content.one().map(<bool>::from),
                19 => msg.crank_length = content.one().map(<u8>::from),
                20 => msg.enabled = content.one().map(<bool>::from),
                21 => msg.bike_spd_ant_id_trans_type = content.one().map(<u8>::from),
                22 => msg.bike_cad_ant_id_trans_type = content.one().map(<u8>::from),
                23 => msg.bike_spdcad_ant_id_trans_type = content.one().map(<u8>::from),
                24 => msg.bike_power_ant_id_trans_type = content.one().map(<u8>::from),
                37 => msg.odometer_rollover = content.one().map(<u8>::from),
                38 => msg.front_gear_num = content.one().map(<u8>::from),
                39 => msg.front_gear = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                40 => msg.rear_gear_num = content.one().map(<u8>::from),
                41 => msg.rear_gear = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                44 => msg.shimano_di2_enabled = content.one().map(<bool>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

