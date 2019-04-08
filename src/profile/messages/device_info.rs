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
pub struct DeviceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    ant_device_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ant_network: Option<crate::profile::enums::AntNetwork>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ant_transmission_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    battery_status: Option<crate::profile::enums::BatteryStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    battery_voltage: Option<crate::fields::ElectricPotential>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cum_operating_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hardware_version: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sensor_position: Option<crate::profile::enums::BodyLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    software_version: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<crate::profile::enums::SourceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl DeviceInfo {
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
                self.device_index =field.one().map(|v| {
                    let value = crate::profile::enums::DeviceIndex::from(v);
                    value
                })
            },

            1 => {
                self.device_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.manufacturer =field.one().map(|v| {
                    let value = crate::profile::enums::Manufacturer::from(v);
                    value
                })
            },

            3 => {
                self.serial_number =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            4 => {
                self.product =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            5 => {
                self.software_version =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            6 => {
                self.hardware_version =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            7 => {
                self.cum_operating_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u32>)(value)
                })
            },

            10 => {
                self.battery_voltage =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::ElectricPotential::new::<uom::si::electric_potential::volt, f64>((|v| { f64::from(v) / 256.0 - 0.0 })(v)))(value)
                })
            },

            11 => {
                self.battery_status =field.one().map(|v| {
                    let value = crate::profile::enums::BatteryStatus::from(v);
                    value
                })
            },

            18 => {
                self.sensor_position =field.one().map(|v| {
                    let value = crate::profile::enums::BodyLocation::from(v);
                    value
                })
            },

            19 => {
                self.descriptor =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            20 => {
                self.ant_transmission_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            21 => {
                self.ant_device_number =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            22 => {
                self.ant_network =field.one().map(|v| {
                    let value = crate::profile::enums::AntNetwork::from(v);
                    value
                })
            },

            25 => {
                self.source_type =field.one().map(|v| {
                    let value = crate::profile::enums::SourceType::from(v);
                    value
                })
            },

            27 => {
                self.product_name =field.one().map(|v| {
                    let value = String::from(v);
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
