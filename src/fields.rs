use std::io::{Cursor, Read};
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub enum FieldContent {
    Enum(u8),
    SignedInt8(i8),
    UnsignedInt8(u8),
    SignedInt16(i16),
    UnsignedInt16(u16),
    SignedInt32(i32),
    UnsignedInt32(u32),
    String(String),
    Float32(f32),
    Float64(f64),
    UnsignedInt8z(u8),
    UnsignedInt16z(u16),
    UnsignedInt32z(u32),
    ByteArray(Vec<u8>),
    SignedInt64(i64),
    UnsignedInt64(u64),
    UnsignedInt64z(u64),
    Invalid,
}

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    number: u8,
    size: usize,
    base_type: u8,
}

impl FieldDefinition {
    pub fn new(number: u8, size: usize, base_type: u8) -> FieldDefinition {
        FieldDefinition {
            number,
            size,
            base_type
        }
    }

    pub fn content_from_bytes<'i, Order: byteorder::ByteOrder>(&self, bytes: &'i [u8])
        -> Result<(&'i [u8], Vec<FieldContent>), std::io::Error>
    {
        let mut cursor = Cursor::new(bytes);
        let data = (0..self.value_count()).map({ |_|
            match self.base_type {
                0 => Ok(FieldContent::Enum(cursor.read_u8()?)),
                1 => Ok(FieldContent::SignedInt8(cursor.read_i8()?)),
                2 => Ok(FieldContent::UnsignedInt8(cursor.read_u8()?)),
                3 => Ok(FieldContent::SignedInt16(cursor.read_i16::<Order>()?)),
                4 => Ok(FieldContent::UnsignedInt16(cursor.read_u16::<Order>()?)),
                5 => Ok(FieldContent::SignedInt32(cursor.read_i32::<Order>()?)),
                6 => Ok(FieldContent::UnsignedInt32(cursor.read_u32::<Order>()?)),
                7 => {
                    let mut data = vec![0; self.size];
                    cursor.read_exact(&mut data)?;

                    let mut iter = data.splitn(2, |b| *b == 0);
                    let string = String::from_utf8_lossy(iter.next().expect("should have at least one item"));
                    Ok(FieldContent::String(string.into_owned()))
                },
                8 => Ok(FieldContent::Float32(cursor.read_f32::<Order>()?)),
                9 => Ok(FieldContent::Float64(cursor.read_f64::<Order>()?)),
                10 => Ok(FieldContent::UnsignedInt8z(cursor.read_u8()?)),
                11 => Ok(FieldContent::UnsignedInt16z(cursor.read_u16::<Order>()?)),
                12 => Ok(FieldContent::UnsignedInt32z(cursor.read_u32::<Order>()?)),
                13 => {
                    let mut data = Vec::with_capacity(self.size);
                    cursor.read_exact(&mut data)?;
                    Ok(FieldContent::ByteArray(data))
                },
                14 => Ok(FieldContent::SignedInt64(cursor.read_i64::<Order>()?)),
                15 => Ok(FieldContent::UnsignedInt64(cursor.read_u64::<Order>()?)),
                16 => Ok(FieldContent::UnsignedInt64z(cursor.read_u64::<Order>()?)),
                _ => panic!("impossible base type: {}", self.base_type),
            }
        }).map({ |content: Result<_, std::io::Error>|
            content.unwrap_or(FieldContent::Invalid)
        }).collect();

        Ok((&bytes[self.size..], data))
    }

    pub fn base_size(&self) -> usize {
        match self.base_type {
            0 => 1,
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 2,
            5 => 4,
            6 => 4,
            7 => 1,
            8 => 4,
            9 => 8,
            10 => 1,
            11 => 2,
            12 => 4,
            13 => 1,
            14 => 8,
            15 => 8,
            16 => 8,
            _ => panic!("impossible base type: {}", self.base_type),
        }
    }

    pub fn value_count(&self) -> usize {
        match self.base_type {
            7 => 1,
            13 => 1,
            _ => self.size / self.base_size(),
        }
    }
}