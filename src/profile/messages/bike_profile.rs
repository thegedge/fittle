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
pub struct BikeProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_power_zero: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_wheel_cal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_wheelsize: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_cad_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_cad_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_power_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_power_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spd_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spd_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spdcad_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_spdcad_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bike_weight: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cad_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    crank_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_wheelsize: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer_rollover: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_cal_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shimano_di2_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spd_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spdcad_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,
}

impl BikeProfile {
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
                self.name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            2 => {
                self.sub_sport =field.one().map(|v| {
                    let value = crate::profile::enums::SubSport::from(v);
                    value
                })
            },

            3 => {
                self.odometer =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.bike_spd_ant_id =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            5 => {
                self.bike_cad_ant_id =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            6 => {
                self.bike_spdcad_ant_id =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            7 => {
                self.bike_power_ant_id =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            8 => {
                self.custom_wheelsize =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            9 => {
                self.auto_wheelsize =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            10 => {
                self.bike_weight =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            11 => {
                self.power_cal_factor =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            12 => {
                self.auto_wheel_cal =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            13 => {
                self.auto_power_zero =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            14 => {
                self.id =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            15 => {
                self.spd_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            16 => {
                self.cad_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            17 => {
                self.spdcad_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            18 => {
                self.power_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            19 => {
                self.crank_length =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { f64::from(v) / 2.0 - -110.0 })(v)))(value)
                })
            },

            20 => {
                self.enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            21 => {
                self.bike_spd_ant_id_trans_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            22 => {
                self.bike_cad_ant_id_trans_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            23 => {
                self.bike_spdcad_ant_id_trans_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            24 => {
                self.bike_power_ant_id_trans_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            37 => {
                self.odometer_rollover =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            38 => {
                self.front_gear_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            39 => {
                self.front_gear =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            40 => {
                self.rear_gear_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            41 => {
                self.rear_gear =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            44 => {
                self.shimano_di2_enabled =field.one().map(|v| {
                    let value = bool::from(v);
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
