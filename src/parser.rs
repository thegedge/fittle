use std::result;
use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;
use nom::{le_u8, be_u16, le_u16, le_u32};

use crate::messages::{MessageType};

#[derive(Debug)]
pub enum ParserError {
    FailedToParse,
}

pub type Result = result::Result<(), ParserError>;

//const MESSAGE_TYPE_DATA: u8 = 0;
const MESSAGE_TYPE_DEFINITION: u8 = 1;

//const LITTLE_ENDIANNESS: u8 = 0;
const BIG_ENDIANNESS: u8 = 1;

#[derive(Debug)]
struct FileHeader {
    size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: usize,
    data_type: [u8; 4],
    crc: u16,
}

#[derive(Debug)]
struct NormalHeader {
    message_type: u8,
    message_type_specific: u8,
    local_message_type: u8,
}

#[derive(Debug)]
struct CompressedHeader {
    local_message_type: u8,
    offset: u8,
}

#[derive(Debug)]
enum RecordHeader {
    Normal(NormalHeader),
    Compressed(CompressedHeader),
}

impl RecordHeader {
    fn is_definition(&self) -> bool {
        match self {
            RecordHeader::Normal(NormalHeader { message_type: MESSAGE_TYPE_DEFINITION, .. }) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Field {
    number: u8,
    size: usize,
    base_type: u8,
}

impl Field {
    fn base_size(&self) -> usize {
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

    fn value_count(&self) -> usize {
        match self.base_type {
            7 => 1,
            13 => 1,
            _ => self.size / self.base_size(),
        }
    }
}

#[derive(Clone, Debug)]
struct DataDefinition {
    architecture: u8,
    global_message: MessageType,
    fields: Vec<Field>,
}

named!(
    normal_header<(&[u8], usize), RecordHeader>,
    do_parse!(
        _header_type: tag_bits!(u8, 1, 0)
        >> message_type: take_bits!(u8, 1)
        >> message_type_specific: take_bits!(u8, 1)
        >> _reserved: take_bits!(u8, 1)
        >> local_message_type: take_bits!(u8, 4)
        >> (RecordHeader::Normal(NormalHeader { message_type, message_type_specific, local_message_type }))
    )
);

named!(
    compressed_header<(&[u8], usize), RecordHeader>,
    do_parse!(
        _header_type: tag_bits!(u8, 1, 1)
        >> local_message_type: take_bits!(u8, 2)
        >> offset: take_bits!(u8, 5)
        >> (RecordHeader::Compressed(CompressedHeader { local_message_type, offset }))
    )
);

named!(
    record_header<&[u8], RecordHeader>,
    bits!(alt!(normal_header | compressed_header))
);

#[derive(Debug)]
enum FieldContent {
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

named_args!(
    fields(num_fields: usize)<&[u8], Vec<Field>>,
    many_m_n!(
        num_fields,
        num_fields,
        do_parse!(
            number: le_u8
            >> size: map!(le_u8, |b| b as usize)
            >> base_type: map!(le_u8, |b| b & 0x0F) // just take the bits for the base type number
            >> (Field { number, size, base_type })
        )
    )
);

fn data_definition<'i>(input: &'i [u8]) -> nom::IResult<&'i [u8], DataDefinition> {
    let _reserved = input[0];
    let architecture = input[1];
    let (_, global_message_number) = if architecture == BIG_ENDIANNESS {
        be_u16(&input[2..])?
    } else {
        le_u16(&input[2..])?
    };

    let num_fields = input[4] as usize;
    let (remaining, fields) = fields(&input[5..], num_fields)?;

    Ok((
        remaining,
        DataDefinition {
            architecture,
            global_message: MessageType::from(global_message_number),
            fields
        }
    ))
}

fn field_content<'i, Order: byteorder::ByteOrder>(input: &'i [u8], field: &Field)
    -> result::Result<(&'i [u8], Vec<FieldContent>), std::io::Error>
{
    let mut cursor = Cursor::new(input);
    let data = (0..field.value_count()).map({ |_|
        match field.base_type {
            0 => Ok(FieldContent::Enum(cursor.read_u8()?)),
            1 => Ok(FieldContent::SignedInt8(cursor.read_i8()?)),
            2 => Ok(FieldContent::UnsignedInt8(cursor.read_u8()?)),
            3 => Ok(FieldContent::SignedInt16(cursor.read_i16::<Order>()?)),
            4 => Ok(FieldContent::UnsignedInt16(cursor.read_u16::<Order>()?)),
            5 => Ok(FieldContent::SignedInt32(cursor.read_i32::<Order>()?)),
            6 => Ok(FieldContent::UnsignedInt32(cursor.read_u32::<Order>()?)),
            7 => {
                let mut data = vec![0; field.size];
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
                let mut data = Vec::with_capacity(field.size);
                cursor.read_exact(&mut data)?;
                Ok(FieldContent::ByteArray(data))
            },
            14 => Ok(FieldContent::SignedInt64(cursor.read_i64::<Order>()?)),
            15 => Ok(FieldContent::UnsignedInt64(cursor.read_u64::<Order>()?)),
            16 => Ok(FieldContent::UnsignedInt64z(cursor.read_u64::<Order>()?)),
            _ => panic!("impossible base type: {}", field.base_type),
        }
    }).map({ |content: result::Result<FieldContent, std::io::Error>|
        content.unwrap_or(FieldContent::Invalid)
    }).collect();
    Ok((&input[field.size..], data))
}

named!(
    file_header<&[u8], FileHeader>,
    do_parse!(
        size: le_u8
        >> protocol_version: le_u8
        >> profile_version: le_u16
        >> data_size: map!(le_u32, |b| b as usize)
        >> data_type: count_fixed!(u8, le_u8, 4)
        >> crc: le_u16
        >> (FileHeader { size, protocol_version, profile_version, data_size, data_type, crc })
    )
);

fn file<'i>(input: &'i [u8]) -> nom::IResult<&'i [u8], ()> {
    let mut local_types = vec![None; 255];

    let (mut remaining, file_header) = file_header(input)?;
    let end_size = remaining.len() - file_header.data_size;
    while remaining.len() > end_size {
        let record_header = record_header(remaining)?;

        let local_message_type = match record_header.1 {
            RecordHeader::Normal(ref h) => h.local_message_type,
            RecordHeader::Compressed(ref h) => h.local_message_type,
        };

        // TODO compressed offsets

        if record_header.1.is_definition() {
            let data_definition = data_definition(record_header.0)?;
            local_types[local_message_type as usize] = Some(data_definition.1);
            remaining = data_definition.0
        } else {
            match local_types[local_message_type as usize] {
                Some(ref data_definition) => {
                    remaining = record_header.0;
                    for field in data_definition.fields.iter() {
                        let field_content = if data_definition.architecture == BIG_ENDIANNESS {
                            field_content::<byteorder::BE>(remaining, field)
                        } else {
                            field_content::<byteorder::LE>(remaining, field)
                        }.unwrap();

                        println!("\t{} (size {}) = {:?}", field.number, field.size, field_content.1);
                        remaining = field_content.0;
                    }
                },
                None => panic!("local message type {} not yet defined", local_message_type),
            }
        }
    }

    // TODO CRC verification

    remaining = &remaining[2..];

    Ok((remaining, ()))
}

pub fn parse(bytes: &[u8]) -> Result {
    println!("{:?}", file(bytes));
    Ok(())
}

impl std::convert::From<nom::Err<(&[u8], usize)>> for ParserError {
    fn from(_: nom::Err<(&[u8], usize)>) -> Self {
        ParserError::FailedToParse
    }
}
