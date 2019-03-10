// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct FieldDescription {
    developer_data_index: Option<u8>,
    field_definition_number: Option<u8>,
    fit_base_type_id: Option<enums::FitBaseType>,
    field_name: Option<Vec<String>>,
    array: Option<u8>,
    components: Option<String>,
    scale: Option<u8>,
    offset: Option<i8>,
    units: Option<Vec<String>>,
    bits: Option<String>,
    accumulate: Option<String>,
    fit_base_unit_id: Option<enums::FitBaseUnit>,
    native_mesg_num: Option<enums::MesgNum>,
    native_field_num: Option<u8>,
}

impl FieldDescription {
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
                0 => msg.developer_data_index = content.one().map(<u8>::from),
                1 => msg.field_definition_number = content.one().map(<u8>::from),
                2 => msg.fit_base_type_id = content.one().map(<enums::FitBaseType>::from),
                3 => msg.field_name = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                4 => msg.array = content.one().map(<u8>::from),
                5 => msg.components = content.one().map(<String>::from),
                6 => msg.scale = content.one().map(<u8>::from),
                7 => msg.offset = content.one().map(<i8>::from),
                8 => msg.units = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                9 => msg.bits = content.one().map(<String>::from),
                10 => msg.accumulate = content.one().map(<String>::from),
                13 => msg.fit_base_unit_id = content.one().map(<enums::FitBaseUnit>::from),
                14 => msg.native_mesg_num = content.one().map(<enums::MesgNum>::from),
                15 => msg.native_field_num = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

