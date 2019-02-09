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
const TYPES_SHEET_BASE_TYPE_COLUMN: usize = 1;
const TYPES_SHEET_VALUE_NAME_COLUMN: usize = 2;
const TYPES_SHEET_VALUE_COLUMN: usize = 3;
const TYPES_SHEET_COMMENT_COLUMN: usize = 4;

fn main() -> Result<(), Error> {
    let mut code_gen = CodeGen {
        workbook: open_workbook(env!("FIT_PROFILE_PATH")).expect("cannot open fit profile xlsx"),
        types_stream: File::create(format!("{}/types.rs", OUTPUT_DIR)).expect("cannot create types.rs"),
        messages_stream: File::create(format!("{}/messages.rs", OUTPUT_DIR)).expect("cannot create messages.rs"),
        data_types_stream: File::create(format!("{}/data_types.rs", OUTPUT_DIR)).expect("cannot create data_types.rs"),
    };

    code_gen.types_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;
    code_gen.messages_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;
    code_gen.data_types_stream.write(b"// DO NOT EDIT -- generated code\n\n")?;

    code_gen.generate_types()?;
    code_gen.generate_fields()?;
    code_gen.generate_data_types()?;

    Ok(())
}

struct CodeGen<D: Seek + Read, F: Write> {
    workbook: Xlsx<D>,

    types_stream: F,
    messages_stream: F,
    data_types_stream: F,
}

impl <D: Seek + Read, F: Write> CodeGen<D, F> {
    fn generate_types(&mut self) -> Result<(), Error> {
        let range = self.workbook.worksheet_range("Types")??;

        // Skip the header row
        let mut iter = range.rows().skip(1);
        while let Some(row) = iter.next() {
            let type_name = match &row[TYPES_SHEET_TYPE_NAME_COLUMN] {
                DataType::Empty => continue,
                DataType::String(v) => v,
                value => panic!("Unexpected value in type name column: {}", value),
            };

            let mut values = Vec::new();
            while let Some(type_data) = iter.next() {
                if type_data[TYPES_SHEET_VALUE_NAME_COLUMN].is_empty() {
                    break
                }

                // Skip anything marked as deprecated
                if let Some(s) = type_data[TYPES_SHEET_COMMENT_COLUMN].get_string() {
                    if s.to_lowercase().contains("deprecated") {
                        continue
                    }
                }

                let name = match type_data[TYPES_SHEET_VALUE_NAME_COLUMN].get_string()? {
                    // Ignore these two, since they represent a custom range
                    "mfg_range_min" => continue,
                    "mfg_range_max" => continue,

                    // Handle exceptions in names that can't be converted directly into an identifier
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
                };

                let value = &type_data[TYPES_SHEET_VALUE_COLUMN];
                let unwrapped_value = value
                    .get_int().map(|v| v as u64)
                    .or(value.get_float().map(|v| v as u64))
                    .or(value.get_string().and_then(|v| {
                        let trimmed = v.trim_start_matches("0x");
                        u64::from_str_radix(trimmed, 16).ok()
                    }))?;

                values.push((to_pascal_case(name), unwrapped_value));
            }

            if type_name == "mesg_num" {
                write_enum(&mut self.messages_stream, "MessageType", values)?;
            } else {
                write_enum(&mut self.types_stream, &to_pascal_case(&type_name), values)?;
            }
        }

        Ok(())
    }

    fn generate_fields(&mut self) -> Result<(), Error> {
        let range = self.workbook.worksheet_range("Messages")??;

        // Skip the header row
        let mut iter = range.rows().skip(1);
        while let Some(_row) = iter.next() {
        }

        Ok(())
    }

    fn generate_data_types(&mut self) -> Result<(), Error> {
        Ok(())
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
