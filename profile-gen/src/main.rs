#![feature(try_trait)]

use std::convert::From;
use std::fs::File;

use std::io::{
    Read,
    Seek,
    Write,
};

use calamine::{
    self,

    DataType,
    Reader,
    Xlsx,
    XlsxError,

    open_workbook,
};

use inflector::cases::pascalcase::to_pascal_case;

#[derive(Debug)]
pub struct Error {
    wrapped: String
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Error { wrapped: format!("{:?}", err) } }
}

impl From<XlsxError> for Error {
    fn from(err: XlsxError) -> Self { Error { wrapped: format!("{:?}", err) } }
}

impl From<std::option::NoneError> for Error {
    fn from(err: std::option::NoneError) -> Self { Error { wrapped: format!("{:?}", err) } }
}

const OUTPUT_DIR: &'static str = env!("OUTPUT_DIR");

const TYPES_SHEET_TYPE_NAME_COLUMN: usize = 0;
// const TYPES_SHEET_BASE_TYPE_COLUMN: usize = 1;
const TYPES_SHEET_VALUE_NAME_COLUMN: usize = 2;
const TYPES_SHEET_VALUE_COLUMN: usize = 3;
const TYPES_SHEET_COMMENT_COLUMN: usize = 4;

const MESSAGES_SHEET_MESSAGE_NAME_COLUMN: usize = 0;
const MESSAGES_SHEET_FIELD_NUMBER_COLUMN: usize = 1;
const MESSAGES_SHEET_FIELD_NAME_COLUMN: usize = 2;
const MESSAGES_SHEET_FIELD_TYPE_COLUMN: usize = 3;
//const MESSAGES_SHEET_ARRAY_COLUMN: usize = 4;
const MESSAGES_SHEET_COMPONENTS_COLUMN: usize = 5;
const MESSAGES_SHEET_SCALE_COLUMN: usize = 6;
const MESSAGES_SHEET_OFFSET_COLUMN: usize = 7;
const MESSAGES_SHEET_UNITS_COLUMN: usize = 8;
const MESSAGES_SHEET_BITS_COLUMN: usize = 9;
const MESSAGES_SHEET_ACCUMULATE_COLUMN: usize = 10;
const MESSAGES_SHEET_REF_FIELD_NAME_COLUMN: usize = 11;
const MESSAGES_SHEET_REF_FIELD_VALUE_COLUMN: usize = 12;
const MESSAGES_SHEET_COMMENT_COLUMN: usize = 13;

fn main() -> Result<(), Error> {
    let mut code_gen = CodeGen {
        workbook: open_workbook(env!("FIT_PROFILE_PATH")).expect("cannot open fit profile xlsx"),
        enums_stream: File::create(format!("{}/enums.rs", OUTPUT_DIR)).expect("cannot create enums.rs"),
        messages_stream: File::create(format!("{}/messages.rs", OUTPUT_DIR)).expect("cannot create messages.rs"),
        data_types_stream: File::create(format!("{}/data_types.rs", OUTPUT_DIR)).expect("cannot create data_types.rs"),
    };

    code_gen.enums_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;
    code_gen.messages_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;
    code_gen.data_types_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;

    code_gen.generate_types()?;
    code_gen.generate_fields()?;
    code_gen.generate_data_types()?;

    Ok(())
}

struct CodeGen<D: Seek + Read, F: Write> {
    workbook: Xlsx<D>,

    enums_stream: F,
    messages_stream: F,
    data_types_stream: F,
}

impl <D: Seek + Read, F: Write> CodeGen<D, F> {
    fn generate_types(&mut self) -> Result<(), Error> {
        let range = self.workbook.worksheet_range("Types")??;

        // Skip the header row
        let mut iter = range.rows().skip(1).peekable();
        while let Some(row) = iter.next() {
            let type_name = match &row[TYPES_SHEET_TYPE_NAME_COLUMN] {
                DataType::Empty => continue,
                DataType::String(v) => v,
                value => panic!("Unexpected value in type name column: {}", value),
            };

            let mut values = Vec::new();
            loop {
                if let Some(peek_data) = iter.peek() {
                    if !peek_data[TYPES_SHEET_TYPE_NAME_COLUMN].is_empty() {
                        break
                    }
                } else {
                    break;
                }

                let type_data = iter.next()?;
                if let Some(s) = type_data[TYPES_SHEET_COMMENT_COLUMN].get_string() {
                    if s.to_lowercase().contains("deprecated") {
                        continue
                    }
                }

                if type_data[TYPES_SHEET_VALUE_NAME_COLUMN].is_empty() {
                    break
                }

                let name = match &type_data[TYPES_SHEET_VALUE_NAME_COLUMN] {
                    DataType::Empty => break,
                    DataType::String(v) => match message_name(&v) {
                        Some(v) => v,
                        None => continue,
                    },
                    v => panic!("unexpected type in enum name column: {:?}", v),
                };

                let value = match &type_data[TYPES_SHEET_VALUE_COLUMN] {
                    DataType::Int(v) => *v as u64,
                    DataType::Float(v) => *v as u64,
                    DataType::String(v) => {
                        let trimmed = v.trim_start_matches("0x");
                        u64::from_str_radix(trimmed, 16).ok()?
                    },
                    v => panic!("unexpected type in enum value column: {:?}", v),
                };

                values.push((to_pascal_case(name), value));
            }

            match type_name.as_str() {
                // TODO these have to be implemented differently
                "sport_bits_0" => (),
                "sport_bits_1" => (),
                "sport_bits_2" => (),
                "sport_bits_3" => (),
                "sport_bits_4" => (),
                "sport_bits_5" => (),
                "sport_bits_6" => (),
                "language_bits_0" => (),
                "language_bits_1" => (),
                "language_bits_2" => (),
                "language_bits_3" => (),
                "language_bits_4" => (),
                v => write_enum(&mut self.enums_stream, &to_pascal_case(v), values)?,
            }
        }

        Ok(())
    }

    fn generate_fields(&mut self) -> Result<(), Error> {
        let range = self.workbook.worksheet_range("Messages")??;

        self.messages_stream.write(b"use crate::enums;\n\n")?;

        // Skip the header row
        let mut iter = range.rows().skip(1);
        while let Some(row) = iter.next() {
            let message_name = match &row[MESSAGES_SHEET_MESSAGE_NAME_COLUMN] {
                DataType::Empty => continue,
                DataType::String(v) => to_pascal_case(v),
                value => panic!("Unexpected value in message name column: {}", value),
            };

            let mut values = Vec::new();
            while let Some(field_data) = iter.next() {
                if field_data[MESSAGES_SHEET_FIELD_NAME_COLUMN].is_empty() {
                    break
                }

                match &field_data[MESSAGES_SHEET_COMMENT_COLUMN] {
                    DataType::String(v) => {
                        match v.as_str() {
                            "Use language_bits_x types where x is index of array." => continue,
                            "Use sport_bits_x types where x is index of array." => continue,
                            _ => (),
                        }
                    },
                    _ => (),
                };

                let field_name = match &field_data[MESSAGES_SHEET_FIELD_NAME_COLUMN] {
                    DataType::Empty => break,
                    DataType::String(v) => v,
                    v => panic!("unexpected type in message field name column: {:?}", v),
                };

                let field_type = match &field_data[MESSAGES_SHEET_FIELD_TYPE_COLUMN] {
                    DataType::Empty => break,
                    DataType::String(v) => v,
                    v => panic!("unexpected type in message field name column: {:?}", v),
                };

                let field_number = match &field_data[MESSAGES_SHEET_FIELD_NUMBER_COLUMN] {
                    DataType::Empty => continue,
                    DataType::Int(v) => *v as u64,
                    DataType::Float(v) => *v as u64,
                    DataType::String(v) => {
                        let trimmed = v.trim_start_matches("0x");
                        u64::from_str_radix(trimmed, 16).ok()?
                    },
                    v => panic!("unexpected type in message field number column: {:?}", v),
                };

                values.push((to_pascal_case(&field_name), field_type, field_number));
            }

            self.messages_stream.write_fmt(format_args!("pub struct {0} {{\n", message_name))?;
            for (field_name, field_type, _number) in values {
                self.messages_stream.write_fmt(format_args!("    {0}: {1},\n", field_name, rust_type(&field_type)))?;
            };
            self.messages_stream.write(b"}\n\n")?;
        }

        Ok(())
    }

    fn generate_data_types(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

fn rust_type(field_type: &str) -> String {
    match field_type {
        "sint8" => "i8".to_owned(),
        "sint16" => "i16".to_owned(),
        "sint32" => "i32".to_owned(),
        "sint64" => "i16".to_owned(),
        "uint8" | "uint8z" | "byte" => "u8".to_owned(),
        "uint16" | "uint16z" => "u16".to_owned(),
        "uint32" | "uint32z" => "u32".to_owned(),
        "uint64" | "uint64z" => "u64".to_owned(),
        "float32" => "f32".to_owned(),
        "float64" => "f64".to_owned(),
        "bool" => "bool".to_owned(),
        "string" => "String".to_owned(),
        v => format!("enums::{}", to_pascal_case(v)),
    }
}

fn message_name(name: &str) -> Option<&str> {
    // Handle exceptions in names that can't be converted directly into an identifier
    match name {
        // Ignore these two, since they represent a custom range
        "mfg_range_min" => None,
        "mfg_range_max" => None,
        _ => Some(match name {
            "1partcarbon" => "one_part_carbon",
            "4iiiis" => "four_i_i_i_i",
            "3_way_calf_raise" => "three_way_calf_raise",
            "3_way_weighted_calf_raise" => "three_way_weighted_calf_raise",
            "3_way_single_leg_calf_raise" => "three_way_single_leg_calf_raise",
            "3_way_weighted_single_leg_calf_raise" => "three_way_weighted_single_leg_calf_raise",
            "45_degree_cable_external_rotation" => "fourty_five_degree_cable_external_rotation",
            "45_degree_plank" => "fourty_five_degree_plank",
            "90_degree_static_hold" => "ninety_degree_plank",
            "30_degree_lat_pulldown" => "thirty_degree_lat_pulldown",
            "90_degree_cable_external_rotation" => "ninety_degree_cable_external_rotation",
            v => v,
        })
    }
}

fn write_enum<W>(stream: &mut W, enum_name: &str, mut entries: Vec<(String, u64)>) -> Result<(), Error>
    where
        W: Write,
{
    // Write the message enum (sorted by name)
    entries.sort();

    stream.write(b"#[derive(Clone, Debug)]\n")?;
    stream.write_fmt(format_args!("pub enum {} {{\n", enum_name))?;
    for (message_name, _) in &entries {
        stream.write_fmt(format_args!("    {0},\n", message_name))?;
    }
    stream.write(b"    UnknownValue(u64),\n")?;
    stream.write(b"}\n\n")?;

    // Write function to map the message number (sorted by value) to the message enum type
    entries.sort_by_key(|(_, v)| *v);

    stream.write_fmt(format_args!("impl<N> From<N> for {}\n", enum_name))?;
    stream.write_fmt(format_args!("    where N: Into<u64>\n"))?;
    stream.write_fmt(format_args!("{{\n"))?;
    stream.write_fmt(format_args!("    fn from(number: N) -> {} {{\n", enum_name))?;
    stream.write(b"        match number.into() {\n")?;
    for (message_name, value) in entries {
        stream.write_fmt(format_args!("            {0} => {2}::{1},\n", value, message_name, enum_name))?;
    }
    stream.write_fmt(format_args!("            n => {0}::UnknownValue(n)\n", enum_name))?;
    stream.write(b"        }\n")?;
    stream.write(b"    }\n")?;
    stream.write(b"}\n\n")?;

    Ok(())
}
