// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct DeviceInfo {
    timestamp: Option<enums::DateTime>,
    device_index: Option<enums::DeviceIndex>,
    device_type: Option<u8>,
    manufacturer: Option<enums::Manufacturer>,
    serial_number: Option<u32>,
    product: Option<u16>,
    software_version: Option<u16>,
    hardware_version: Option<u8>,
    cum_operating_time: Option<u32>,
    battery_voltage: Option<u16>,
    battery_status: Option<enums::BatteryStatus>,
    sensor_position: Option<enums::BodyLocation>,
    descriptor: Option<String>,
    ant_transmission_type: Option<u8>,
    ant_device_number: Option<u16>,
    ant_network: Option<enums::AntNetwork>,
    source_type: Option<enums::SourceType>,
    product_name: Option<String>,
}

impl DeviceInfo {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.device_index = content.one().map(<enums::DeviceIndex>::from),
                1 => msg.device_type = content.one().map(<u8>::from),
                2 => msg.manufacturer = content.one().map(<enums::Manufacturer>::from),
                3 => msg.serial_number = content.one().map(<u32>::from),
                4 => msg.product = content.one().map(<u16>::from),
                5 => msg.software_version = content.one().map(<u16>::from),
                6 => msg.hardware_version = content.one().map(<u8>::from),
                7 => msg.cum_operating_time = content.one().map(<u32>::from),
                10 => msg.battery_voltage = content.one().map(<u16>::from),
                11 => msg.battery_status = content.one().map(<enums::BatteryStatus>::from),
                18 => msg.sensor_position = content.one().map(<enums::BodyLocation>::from),
                19 => msg.descriptor = content.one().map(<String>::from),
                20 => msg.ant_transmission_type = content.one().map(<u8>::from),
                21 => msg.ant_device_number = content.one().map(<u16>::from),
                22 => msg.ant_network = content.one().map(<enums::AntNetwork>::from),
                25 => msg.source_type = content.one().map(<enums::SourceType>::from),
                27 => msg.product_name = content.one().map(<String>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

