// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

impl From<Vec<(u8, Field)>> for BikeProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.sport = field.one().map(<enums::Sport>::from),
                2 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                3 => msg.odometer = field.one().map(<u32>::from),
                4 => msg.bike_spd_ant_id = field.one().map(<u16>::from),
                5 => msg.bike_cad_ant_id = field.one().map(<u16>::from),
                6 => msg.bike_spdcad_ant_id = field.one().map(<u16>::from),
                7 => msg.bike_power_ant_id = field.one().map(<u16>::from),
                8 => msg.custom_wheelsize = field.one().map(<u16>::from),
                9 => msg.auto_wheelsize = field.one().map(<u16>::from),
                10 => msg.bike_weight = field.one().map(<u16>::from),
                11 => msg.power_cal_factor = field.one().map(<u16>::from),
                12 => msg.auto_wheel_cal = field.one().map(<bool>::from),
                13 => msg.auto_power_zero = field.one().map(<bool>::from),
                14 => msg.id = field.one().map(<u8>::from),
                15 => msg.spd_enabled = field.one().map(<bool>::from),
                16 => msg.cad_enabled = field.one().map(<bool>::from),
                17 => msg.spdcad_enabled = field.one().map(<bool>::from),
                18 => msg.power_enabled = field.one().map(<bool>::from),
                19 => msg.crank_length = field.one().map(<u8>::from),
                20 => msg.enabled = field.one().map(<bool>::from),
                21 => msg.bike_spd_ant_id_trans_type = field.one().map(<u8>::from),
                22 => msg.bike_cad_ant_id_trans_type = field.one().map(<u8>::from),
                23 => msg.bike_spdcad_ant_id_trans_type = field.one().map(<u8>::from),
                24 => msg.bike_power_ant_id_trans_type = field.one().map(<u8>::from),
                37 => msg.odometer_rollover = field.one().map(<u8>::from),
                38 => msg.front_gear_num = field.one().map(<u8>::from),
                39 => msg.front_gear = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                40 => msg.rear_gear_num = field.one().map(<u8>::from),
                41 => msg.rear_gear = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                44 => msg.shimano_di2_enabled = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

