#![feature(try_trait)]

mod field_data;
mod helpers;

use std::{
    collections::BTreeMap,
    convert::From,
    fs::File,
    io::{
        Read,
        Seek,
    },
};

use calamine::{
    self,

    DataType,
    Reader,
    Xlsx,
    XlsxError,

    open_workbook,
};

use field_data::FieldData;

use handlebars::{
    Handlebars,
    RenderError,
    TemplateError,
};

use inflector::cases::pascalcase::to_pascal_case;

use serde::Serialize;

const OUTPUT_DIR: &'static str = "src/profile";

const TYPES_SHEET_TYPE_NAME_COLUMN: usize = 0;
const TYPES_SHEET_BASE_TYPE_COLUMN: usize = 1;
const TYPES_SHEET_VALUE_NAME_COLUMN: usize = 2;
const TYPES_SHEET_VALUE_COLUMN: usize = 3;
const TYPES_SHEET_COMMENT_COLUMN: usize = 4;

const MESSAGES_SHEET_MESSAGE_NAME_COLUMN: usize = 0;
const MESSAGES_SHEET_FIELD_NUMBER_COLUMN: usize = 1;
const MESSAGES_SHEET_FIELD_NAME_COLUMN: usize = 2;
const MESSAGES_SHEET_FIELD_TYPE_COLUMN: usize = 3;
const MESSAGES_SHEET_ARRAY_COLUMN: usize = 4;
//const MESSAGES_SHEET_COMPONENTS_COLUMN: usize = 5;
const MESSAGES_SHEET_SCALE_COLUMN: usize = 6;
const MESSAGES_SHEET_OFFSET_COLUMN: usize = 7;
const MESSAGES_SHEET_UNITS_COLUMN: usize = 8;
//const MESSAGES_SHEET_BITS_COLUMN: usize = 9;
//const MESSAGES_SHEET_ACCUMULATE_COLUMN: usize = 10;
//const MESSAGES_SHEET_REF_FIELD_NAME_COLUMN: usize = 11;
//const MESSAGES_SHEET_REF_FIELD_VALUE_COLUMN: usize = 12;
const MESSAGES_SHEET_COMMENT_COLUMN: usize = 13;

#[derive(Debug)]
pub struct Error {
    wrapped: String
}

#[derive(Clone, Serialize)]
struct FittleEnumVariant {
    name: String,
    value: u64,
}

#[derive(Clone, Serialize)]
struct FittleEnum {
    name: String,
    module: String,
    base_type: String,
    variants: Vec<FittleEnumVariant>,
}

#[derive(Clone, Serialize)]
struct FittleMessageField {
    name: String,
    number: u64,
    rust_type: String,
    conversion_function: String,
}

#[derive(Clone, Serialize)]
struct FittleMessage {
    name: String,
    module: String,
    fields: BTreeMap<u64, FittleMessageField>,
}

fn main() -> Result<(), Error> {
    let mut renderer = Handlebars::new();
    renderer.set_strict_mode(true);
    renderer.register_escape_fn(str::to_owned);
    renderer.register_template_string("enum", include_str!("../templates/enum.handlebars"))?;
    renderer.register_template_string("enums_mod", include_str!("../templates/enums_mod.handlebars"))?;
    renderer.register_template_string("message", include_str!("../templates/message.handlebars"))?;
    renderer.register_template_string("messages_mod", include_str!("../templates/messages_mod.handlebars"))?;

    renderer.register_helper("sorted", Box::new(helpers::sorted));

    let mut workbook = open_workbook(env!("FIT_PROFILE_PATH")).expect("cannot open fit profile xlsx");

    let enums = enums(&mut workbook)?;
    let enums_stream = File::create(format!("{}/enums/mod.rs", OUTPUT_DIR)).expect("cannot create enums/mod.rs");
    renderer.render_to_write("enums_mod", &enums, enums_stream)?;

    for enum_ in enums.iter() {
        let enum_stream =
            File::create(format!("{}/enums/{}.rs", OUTPUT_DIR, enum_.module))
                .expect("cannot create enums/<enum>.rs");

        renderer.render_to_write("enum", &enum_, enum_stream)?;
    }

    let messages = messages(&mut workbook)?;
    let messages_stream = File::create(format!("{}/messages/mod.rs", OUTPUT_DIR)).expect("cannot create messages/mod.rs");
    renderer.render_to_write("messages_mod", &messages, messages_stream)?;

    for message in messages.iter() {
        let message_stream =
            File::create(format!("{}/messages/{}.rs", OUTPUT_DIR, message.module))
                .expect("cannot create messages/<msg>.rs");

        renderer.render_to_write("message", &message, message_stream)?;
    }

    Ok(())
}

fn enums<D: Seek + Read>(workbook: &mut Xlsx<D>) -> Result<Vec<FittleEnum>, Error> {
    let range = workbook.worksheet_range("Types")??;
    let mut enums = Vec::new();

    // Skip the header row. We need it to be peekable because sometimes there's no empty row
    // between messages, so we peek to check for this.
    let mut iter = range.rows().skip(1).peekable();
    while let Some(row) = iter.next() {
        let type_name = match &row[TYPES_SHEET_TYPE_NAME_COLUMN] {
            DataType::Empty => continue,
            DataType::String(v) => v,
            value => panic!("Unexpected value in type name column: {}", value),
        };

        let base_type = match &row[TYPES_SHEET_BASE_TYPE_COLUMN] {
            DataType::Empty => continue,
            DataType::String(v) => v,
            value => panic!("Unexpected value in base type column: {}", value),
        };

        let mut variants = Vec::new();
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

            variants.push(FittleEnumVariant { name: to_pascal_case(name), value });
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

            // These are not real enums (just specify some constraints)
            "weight" => (),
            "date_time" => (),
            "local_date_time" => (),

            v => {
                enums.push(FittleEnum {
                    name: to_pascal_case(v),
                    module: v.to_owned(),
                    base_type: field_content_type(base_type).to_owned(),
                    variants,
                });
            },
        }
    }

    enums.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(enums)
}

fn messages<D: Seek + Read>(workbook: &mut Xlsx<D>) -> Result<Vec<FittleMessage>, Error> {
    let range = workbook.worksheet_range("Messages")??;
    let mut messages = Vec::new();

    // Skip the header row in the spreadsheet
    let mut iter = range.rows().skip(1).peekable();
    while let Some(row) = iter.next() {
        let message_name = match &row[MESSAGES_SHEET_MESSAGE_NAME_COLUMN] {
            DataType::Empty => continue,
            DataType::String(v) => v,
            value => panic!("Unexpected value in message name column: {}", value),
        };

        // We use a peekable iterator because sometimes there's no empty line between messages
        // in the spreadsheet
        let mut fields = BTreeMap::new();
        loop {
            if let Some(field_data) = iter.peek() {
                if field_data[MESSAGES_SHEET_FIELD_NAME_COLUMN].is_empty() {
                    break
                }
            }

            let field_data = match iter.next() {
                Some(d) => d,
                None => break,
            };

            // TODO figure out how to handle these
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
                DataType::String(v) => if v == "type" { "type_" } else { v },
                v => panic!("unexpected type in message field name column: {:?}", v),
            };

            let field_type = match &field_data[MESSAGES_SHEET_FIELD_TYPE_COLUMN] {
                DataType::Empty => break,
                DataType::String(v) => v.clone(),
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

            let field_array_length = match &field_data[MESSAGES_SHEET_ARRAY_COLUMN] {
                DataType::Empty => None,
                DataType::String(s) => {
                    let without_brackets = &s[1..s.len()-1];
                    match without_brackets {
                        "N" => Some(0),
                        v => u8::from_str_radix(v, 10).ok(),
                    }
                },
                v => panic!("unexpected type in message array column: {:?}", v),
            };

            let field_scale = match &field_data[MESSAGES_SHEET_SCALE_COLUMN] {
                DataType::Empty => None,
                DataType::Int(v) => Some(*v as u16),
                DataType::Float(v) => Some(*v as u16),
                DataType::String(v) => {
                    let trimmed = v.trim_start_matches("0x");
                    u16::from_str_radix(trimmed, 10).ok()
                },
                v => panic!("unexpected type in message scale column: {:?}", v),
            };

            let field_offset = match &field_data[MESSAGES_SHEET_OFFSET_COLUMN] {
                DataType::Empty => None,
                DataType::Int(v) => Some(*v as u16),
                DataType::Float(v) => Some(*v as u16),
                DataType::String(v) => {
                    let trimmed = v.trim_start_matches("0x");
                    u16::from_str_radix(trimmed, 10).ok()
                },
                v => panic!("unexpected type in message offset column: {:?}", v),
            };

            let field_unit = match &field_data[MESSAGES_SHEET_UNITS_COLUMN] {
                DataType::Empty => None,
                DataType::String(v) => Some(v.clone()),
                v => panic!("unexpected type in units column: {:?}", v),
            };

            let field_data = FieldData {
                base_type: field_type,
                array_length: field_array_length,
                scale: field_scale,
                offset: field_offset,
                unit: field_unit,
            };

            fields.insert(field_number, FittleMessageField {
                name: field_name.to_owned(),
                number: field_number,
                rust_type: field_data.rust_type(),
                conversion_function: field_data.conversion_function(),
            });
        }

        messages.push(FittleMessage {
            name: to_pascal_case(message_name),
            module: message_name.clone(),
            fields,
         });
    }

    messages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(messages)
}

fn field_content_type(field_type: &str) -> &str {
    match field_type {
        "enum" => "Enum",
        "sint8" => "SignedInt8",
        "uint8" => "UnsignedInt8",
        "sint16" => "SignedInt16",
        "uint16" => "UnsignedInt16",
        "sint32" => "SignedInt32",
        "uint32" => "UnsignedInt32",
        "string" => "String",
        "float32" => "Float32",
        "float64" => "Float64",
        "uint8z" => "UnsignedInt8z",
        "uint16z" => "UnsignedInt16z",
        "uint32z" => "UnsignedInt32z",
        "byte" => "ByteArray",
        "sint64" => "SignedInt64",
        "uint64" => "UnsignedInt64",
        "uint64z" => "UnsignedInt64z",
        _ => panic!("unknown field content type: {}", field_type),
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
            "4iiiis" => "four_i_i_i_is",
            "3_way_calf_raise" => "three_way_calf_raise",
            "3_way_weighted_calf_raise" => "three_way_weighted_calf_raise",
            "3_way_single_leg_calf_raise" => "three_way_single_leg_calf_raise",
            "3_way_weighted_single_leg_calf_raise" => "three_way_weighted_single_leg_calf_raise",
            "45_degree_cable_external_rotation" => "forty_five_degree_cable_external_rotation",
            "45_degree_plank" => "forty_five_degree_plank",
            "90_degree_static_hold" => "ninety_degree_plank",
            "30_degree_lat_pulldown" => "thirty_degree_lat_pulldown",
            "90_degree_cable_external_rotation" => "ninety_degree_cable_external_rotation",
            v => v,
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { wrapped: format!("{}", err) }
    }
}

impl From<XlsxError> for Error {
    fn from(err: XlsxError) -> Self {
        Error { wrapped: format!("{}", err) }
    }
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error { wrapped: "unwrapped a None value".to_owned() }
    }
}

impl From<RenderError> for Error {
    fn from(err: RenderError) -> Self {
        Error { wrapped: format!("{} (in template `{}`)", err, err.template_name.as_ref().unwrap_or(&"unknown".to_string())) }
    }
}

impl From<TemplateError> for Error {
    fn from(err: TemplateError) -> Self {
        Error { wrapped: format!("{} (in template `{}`)", err, err.template_name.as_ref().unwrap_or(&"unknown".to_string())) }
    }
}
