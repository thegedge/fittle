// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

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
    battery_voltage: Option<f64>,

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
                0 => msg.device_index = content.one().map(<crate::profile::enums::DeviceIndex>::from),
                1 => msg.device_type = content.one().map(<u8>::from),
                2 => msg.manufacturer = content.one().map(<crate::profile::enums::Manufacturer>::from),
                3 => msg.serial_number = content.one().map(<u32>::from),
                4 => msg.product = content.one().map(<u16>::from),
                5 => msg.software_version = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                6 => msg.hardware_version = content.one().map(<u8>::from),
                7 => msg.cum_operating_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))),
                10 => msg.battery_voltage = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 256.0 - 0.0 }),
                11 => msg.battery_status = content.one().map(<crate::profile::enums::BatteryStatus>::from),
                18 => msg.sensor_position = content.one().map(<crate::profile::enums::BodyLocation>::from),
                19 => msg.descriptor = content.one().map(<String>::from),
                20 => msg.ant_transmission_type = content.one().map(<u8>::from),
                21 => msg.ant_device_number = content.one().map(<u16>::from),
                22 => msg.ant_network = content.one().map(<crate::profile::enums::AntNetwork>::from),
                25 => msg.source_type = content.one().map(<crate::profile::enums::SourceType>::from),
                27 => msg.product_name = content.one().map(<String>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
