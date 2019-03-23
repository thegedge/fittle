use std::collections::HashMap;

use byteorder::{
    ByteOrder,
    ReadBytesExt,
};

use serde::Serialize;

use crate::fields::{
    Field,
    FieldDefinition,
};

#[derive(Debug, Serialize)]
pub struct GenericMessage {
    number: u64,
    fields: HashMap<u8, Field>,
}

impl GenericMessage {
    pub fn new<Order, Reader>(reader: &mut Reader, number: u64, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut fields = HashMap::new();
        for field in field_defs {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            if !content.is_invalid() {
                fields.insert(number, content);
            }
        }

        Ok(GenericMessage { number, fields })
    }
}
