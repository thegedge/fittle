use std::result;
use nom::{le_u8, le_u16, le_u32};

#[derive(Debug)]
pub enum ParserError {
    FailedToParse,
}

pub type Result = result::Result<(), ParserError>;

#[derive(Debug)]
struct FileHeader {
    size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32,
    data_type: [u8; 4],
    crc: u16,
}

named!(
    file_header<&[u8], FileHeader>,
    do_parse!(
        size: le_u8
        >> protocol_version: le_u8
        >> profile_version: le_u16
        >> data_size: le_u32
        >> data_type: count_fixed!(u8, le_u8, 4)
        >> crc: le_u16
        >> (FileHeader { size, protocol_version, profile_version, data_size, data_type, crc })
    )
);

pub fn parse(bytes: &[u8]) -> Result {
    let (_remaining, header) = file_header(bytes)?;
    println!("{:?}", header);
    Ok(())
}

impl std::convert::From<nom::Err<&[u8]>> for ParserError {
    fn from(_: nom::Err<&[u8]>) -> Self {
        ParserError::FailedToParse
    }
}
