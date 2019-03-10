// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct BikeProfile {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    odometer: Option<u32>,
    bike_spd_ant_id: Option<u16>,
    bike_cad_ant_id: Option<u16>,
    bike_spdcad_ant_id: Option<u16>,
    bike_power_ant_id: Option<u16>,
    custom_wheelsize: Option<u16>,
    auto_wheelsize: Option<u16>,
    bike_weight: Option<u16>,
    power_cal_factor: Option<u16>,
    auto_wheel_cal: Option<bool>,
    auto_power_zero: Option<bool>,
    id: Option<u8>,
    spd_enabled: Option<bool>,
    cad_enabled: Option<bool>,
    spdcad_enabled: Option<bool>,
    power_enabled: Option<bool>,
    crank_length: Option<u8>,
    enabled: Option<bool>,
    bike_spd_ant_id_trans_type: Option<u8>,
    bike_cad_ant_id_trans_type: Option<u8>,
    bike_spdcad_ant_id_trans_type: Option<u8>,
    bike_power_ant_id_trans_type: Option<u8>,
    odometer_rollover: Option<u8>,
    front_gear_num: Option<u8>,
    front_gear: Option<Vec<u8>>,
    rear_gear_num: Option<u8>,
    rear_gear: Option<Vec<u8>>,
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

