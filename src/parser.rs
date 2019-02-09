use std::result;

use nom::{le_u8, be_u16, le_u16, le_u32};

use crate::{
    fields::FieldDefinition,
    messages::MessageType,
};

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
struct DataDefinition {
    architecture: u8,
    global_message: MessageType,
    fields: Vec<FieldDefinition>,
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

named_args!(
    fields(num_fields: usize)<&[u8], Vec<FieldDefinition>>,
    many_m_n!(
        num_fields,
        num_fields,
        do_parse!(
            number: le_u8
            >> size: map!(le_u8, |b| b as usize)
            >> base_type: map!(le_u8, |b| b & 0x0F) // just take the bits for the base type number
            >> (FieldDefinition::new(number, size, base_type))
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
                            field.content_from_bytes::<byteorder::BE>(remaining)
                        } else {
                            field.content_from_bytes::<byteorder::LE>(remaining)
                        }.unwrap();

                        println!("\t{:?} = {:?}", field, field_content.1);
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
