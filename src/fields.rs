use byteorder::{
    ByteOrder,
    ReadBytesExt,
};

#[derive(Debug)]
pub enum Field {
    One(FieldContent),
    Many(Vec<FieldContent>),
}

impl Field {
    pub fn one(self) -> Option<FieldContent> {
        match self {
            Field::One(v) => Some(v),
            _ => None
        }
    }

    pub fn many(self) -> Option<Vec<FieldContent>> {
        match self {
            Field::Many(v) => Some(v),
            _ => None
        }
    }
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
        -> Result<(u8, Field), std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        Ok(if self.value_count() == 1 {
            (self.number, Field::One(self.read_one::<Order, Reader>(reader)?))
        } else {
            (
                self.number,
                Field::Many(
                    (0..self.value_count())
                        .map({ |_| self.read_one::<Order, Reader>(reader) })
                        .collect::<Result<Vec<_>, _>>()?
                )
            )
        })
    }

    pub fn read_one<'i, Order, Reader>(&self, reader: &mut Reader)
        -> Result<FieldContent, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
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

macro_rules! into_impl {
    ( $into_type:ty, $( $enums:tt )|+ ) => {
        into_impl!($into_type, $($enums)|+, |v| <$into_type>::from(v));
    };

    ( $into_type:ty, $( $enums:tt )|+, $conversion:expr ) => {
        impl From<FieldContent> for $into_type {
            fn from(fc: FieldContent) -> Self {
                ($conversion)(match fc {
                    $(
                        FieldContent::$enums(v) => v,
                    )*
                    v => panic!("cannot convert {:?} into {}", v, stringify!($type)),
                }) 
            }
        }

        impl From<Field> for $into_type {
            fn from(f: Field) -> Self {
                ($conversion)(match f {
                    $(
                        Field::One(FieldContent::$enums(v)) => v,
                    )*
                    v => panic!("cannot convert {:?} into {}", v, stringify!($type)),
                })
            }
        }
    };
}

into_impl!(bool, UnsignedInt8, |v| v != 0);
into_impl!(u8, Enum | UnsignedInt8 | UnsignedInt8z);
into_impl!(u16, UnsignedInt16 | UnsignedInt16z);
into_impl!(u32, UnsignedInt32 | UnsignedInt32z);
into_impl!(u64, UnsignedInt64 | UnsignedInt64z);
into_impl!(i8, SignedInt8);
into_impl!(i16, SignedInt16);
into_impl!(i32, SignedInt32);
into_impl!(i64, SignedInt64);
into_impl!(f32, Float32);
into_impl!(f64, Float64);
into_impl!(String, String);
into_impl!(Vec<u8>, ByteArray);
