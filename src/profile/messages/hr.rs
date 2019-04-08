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
pub struct Hr {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_timestamp_12: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filtered_bpm: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time256: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl Hr {
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
                self.time256 =field.one().map(|v| {
                    let value = u8::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u16>(8).map(|bits_value| {
                            self.from_content(0, Field::One(FieldContent::UnsignedInt16(bits_value)));
                        });
                    }
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 256.0 - 0.0 })(v)))(value)
                })
            },

            6 => {
                self.filtered_bpm =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>).collect()
                })
            },

            9 => {
                self.event_timestamp =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1024.0 - 0.0 })(v))).collect()
                })
            },

            10 => {
                self.event_timestamp_12 =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    let bits = value.as_slice();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(12).map(|bits_value| {
                            self.from_content(9, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
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
