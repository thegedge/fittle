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
pub struct AntTx {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl AntTx {
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
                self.fractional_timestamp =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 32768.0 - 0.0 })(v)))(value)
                })
            },

            1 => {
                self.mesg_id =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.mesg_data =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    let bits = value.as_slice();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(3, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(8).map(|bits_value| {
                            self.from_content(4, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    value
                })
            },

            3 => {
                self.channel_number =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            4 => {
                self.data =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
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
