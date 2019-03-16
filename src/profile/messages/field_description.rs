// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct FieldDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    accumulate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    array: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bits: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_data_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_definition_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fit_base_type_id: Option<crate::profile::enums::FitBaseType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fit_base_unit_id: Option<crate::profile::enums::FitBaseUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    native_field_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    native_mesg_num: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Vec<String>>,
}

impl FieldDescription {
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
                0 => msg.developer_data_index = content.one().map(<u8>::from),
                1 => msg.field_definition_number = content.one().map(<u8>::from),
                2 => msg.fit_base_type_id = content.one().map(<crate::profile::enums::FitBaseType>::from),
                3 => msg.field_name = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                4 => msg.array = content.one().map(<u8>::from),
                5 => msg.components = content.one().map(<String>::from),
                6 => msg.scale = content.one().map(<u8>::from),
                7 => msg.offset = content.one().map(<i8>::from),
                8 => msg.units = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                9 => msg.bits = content.one().map(<String>::from),
                10 => msg.accumulate = content.one().map(<String>::from),
                13 => msg.fit_base_unit_id = content.one().map(<crate::profile::enums::FitBaseUnit>::from),
                14 => msg.native_mesg_num = content.one().map(<crate::profile::enums::MesgNum>::from),
                15 => msg.native_field_num = content.one().map(<u8>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
