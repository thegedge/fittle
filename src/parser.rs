use std::{
    result,
    io::{
        Cursor,
        Read,
        Seek,
        SeekFrom,
    },
};

use byteorder::{
    BigEndian,
    LittleEndian,
    ReadBytesExt,
};

use crate::{
    fields::{
        FieldContent,
        FieldDefinition,
    },
    enums::MesgNum,
};

#[derive(Debug)]
pub enum ParserError {
    FailedToParse,
}

pub type Result<T> = result::Result<T, ParserError>;

const HEADER_TYPE_NORMAL: u8 = 0;
//const HEADER_TYPE_COMPRESSED: u8 = 1;

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

    fn local_message_type(&self) -> u8 {
        match self {
            RecordHeader::Normal(ref h) => h.local_message_type,
            RecordHeader::Compressed(ref h) => h.local_message_type,
        }
    }
}

#[derive(Clone, Debug)]
struct DataDefinition {
    architecture: u8,
    message_type: MesgNum,
    fields: Vec<FieldDefinition>,
}

struct Parser<Reader: ReadBytesExt> {
    reader: Reader,
}

fn read_bits(value: &mut u8, n: u8) -> u8 {
    let mask = ((1 << n) as u8 - 1).rotate_right(n as u32);
    let ret = (*value & mask) >> (8 - n);
    *value <<= n;
    ret
}

pub fn parse(bytes: &[u8]) -> Result<()> {
    let reader = Cursor::new(bytes);
    Parser { reader }.parse()
}

impl<Reader> Parser<Reader> where Reader: Read + Seek {
    fn record_header(&mut self, mut header_data: u8) -> Result<RecordHeader> {
        let data = &mut header_data;
        let header_type = read_bits(data, 1);
        if header_type == HEADER_TYPE_NORMAL {
            let message_type = read_bits(data, 1);
            let message_type_specific = read_bits(data, 1);
            let _reserved = read_bits(data, 1);
            let local_message_type = read_bits(data, 4);

            Ok(RecordHeader::Normal(NormalHeader { message_type, message_type_specific, local_message_type }))
        } else {
            let local_message_type = read_bits(data, 2);
            let offset = read_bits(data, 5);

            Ok(RecordHeader::Compressed(CompressedHeader { local_message_type, offset }))
        }
    }

    fn fields(&mut self, num_fields: usize) -> Result<Vec<FieldDefinition>> {
        (0..num_fields).map(|_| {
            let number = self.reader.read_u8()?;
            let size = self.reader.read_u8()? as usize;
            let base_type = self.reader.read_u8()? & 0x0F; // just take the bits for the base type number

            Ok(FieldDefinition::new(number, size, base_type))
        }).collect()
    }

    fn data_definition(&mut self) -> Result<DataDefinition> {
        let _reserved = self.reader.read_u8()?;

        let architecture = self.reader.read_u8()?;
        let global_message_number = if architecture == BIG_ENDIANNESS {
            self.reader.read_u16::<BigEndian>()?
        } else {
            self.reader.read_u16::<LittleEndian>()?
        };

        let num_fields = self.reader.read_u8()? as usize;
        let fields = self.fields(num_fields)?;

        Ok(DataDefinition {
            architecture,
            message_type: MesgNum::from(FieldContent::UnsignedInt16(global_message_number)),
            fields
        })
    }

    fn file_header(&mut self) -> Result<FileHeader> {
        let size = self.reader.read_u8()?;
        let protocol_version = self.reader.read_u8()?;
        let profile_version = self.reader.read_u16::<LittleEndian>()?;
        let data_size = self.reader.read_u32::<LittleEndian>()? as usize;
        let data_type = [
            self.reader.read_u8()?,
            self.reader.read_u8()?,
            self.reader.read_u8()?,
            self.reader.read_u8()?,
        ];
        let crc = self.reader.read_u16::<LittleEndian>()?;

        Ok(FileHeader { size, protocol_version, profile_version, data_size, data_type, crc })
    }

    fn parse<'i>(&mut self) -> Result<()> {
        let mut local_types = vec![None; 255];

        let file_header = self.file_header()?;
        let end_position = self.reader.seek(SeekFrom::Current(0))? + file_header.data_size as u64;
        while self.reader.seek(SeekFrom::Current(0))? < end_position {
            let record_header_data = self.reader.read_u8()?;
            let record_header = self.record_header(record_header_data)?;
            let local_message_type = record_header.local_message_type();
            println!("{:?}", record_header);

            // TODO compressed offsets

            if record_header.is_definition() {
                let data_definition = self.data_definition()?;
                local_types[local_message_type as usize] = Some(data_definition);
            } else {
                match local_types[local_message_type as usize] {
                    Some(ref data_definition) => {
                        println!("Message: {:?}", data_definition.message_type);

                        for field in data_definition.fields.iter() {
                            let (_, field_content) = if data_definition.architecture == BIG_ENDIANNESS {
                                field.content_from::<BigEndian, Reader>(&mut self.reader)
                            } else {
                                field.content_from::<LittleEndian, Reader>(&mut self.reader)
                            }.unwrap();

                            println!("\t{:?}", field_content);
                        }
                    },
                    None => panic!("local message type {} not yet defined", local_message_type),
                }
            }
        }

        // TODO CRC verification

        Ok(())
    }
}

impl std::convert::From<std::io::Error> for ParserError {
    fn from(_: std::io::Error) -> Self {
        ParserError::FailedToParse
    }
}
