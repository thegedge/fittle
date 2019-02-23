use byteorder::{
    ByteOrder,
    ReadBytesExt,
};

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

    pub fn content_from<'i, Order, Reader>(&self, reader: &mut Reader)
        -> Result<(Vec<FieldContent>), std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        (0..self.value_count()).map({ |_|
            Ok(match self.base_type {
                0 => FieldContent::Enum(reader.read_u8()?),
                1 => FieldContent::SignedInt8(reader.read_i8()?),
                2 => FieldContent::UnsignedInt8(reader.read_u8()?),
                3 => FieldContent::SignedInt16(reader.read_i16::<Order>()?),
                4 => FieldContent::UnsignedInt16(reader.read_u16::<Order>()?),
                5 => FieldContent::SignedInt32(reader.read_i32::<Order>()?),
                6 => FieldContent::UnsignedInt32(reader.read_u32::<Order>()?),
                7 => {
                    let mut data = vec![0; self.size];
                    reader.read_exact(&mut data)?;

                    let mut iter = data.splitn(2, |b| *b == 0);
                    let string = String::from_utf8_lossy(iter.next().expect("should have at least one item"));
                    FieldContent::String(string.into_owned())
                },
                8 => FieldContent::Float32(reader.read_f32::<Order>()?),
                9 => FieldContent::Float64(reader.read_f64::<Order>()?),
                10 => FieldContent::UnsignedInt8z(reader.read_u8()?),
                11 => FieldContent::UnsignedInt16z(reader.read_u16::<Order>()?),
                12 => FieldContent::UnsignedInt32z(reader.read_u32::<Order>()?),
                13 => {
                    let mut data = vec![0; self.size];
                    reader.read_exact(&mut data)?;
                    FieldContent::ByteArray(data)
                },
                14 => FieldContent::SignedInt64(reader.read_i64::<Order>()?),
                15 => FieldContent::UnsignedInt64(reader.read_u64::<Order>()?),
                16 => FieldContent::UnsignedInt64z(reader.read_u64::<Order>()?),
                _ => panic!("impossible base type: {}", self.base_type),
            })
        }).collect()
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
