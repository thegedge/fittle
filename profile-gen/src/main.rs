#![feature(try_trait)]

use std::convert::From;
use std::fs::File;

use std::io::{
    Read,
    Seek,
};

use calamine::{
    self,

    DataType,
    Reader,
    Xlsx,
    XlsxError,

    open_workbook,
};

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
//const MESSAGES_SHEET_SCALE_COLUMN: usize = 6;
//const MESSAGES_SHEET_OFFSET_COLUMN: usize = 7;
//const MESSAGES_SHEET_UNITS_COLUMN: usize = 8;
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
    variants_sorted_by_value: Vec<FittleEnumVariant>,
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
    fields: Vec<FittleMessageField>,
    fields_sorted_by_number: Vec<FittleMessageField>,
}

fn main() -> Result<(), Error> {
    let mut renderer = Handlebars::new();
    renderer.set_strict_mode(true);
    renderer.register_escape_fn(str::to_owned);
    renderer.register_template_string("enum", include_str!("../templates/enum.handlebars"))?;
    renderer.register_template_string("enums_mod", include_str!("../templates/enums_mod.handlebars"))?;
    renderer.register_template_string("message", include_str!("../templates/message.handlebars"))?;
    renderer.register_template_string("messages_mod", include_str!("../templates/messages_mod.handlebars"))?;

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

        // TODO perhaps have a handlebars helper to do this
        let mut variants_sorted_by_value = variants.clone();

        variants.sort_by(|a, b| a.name.cmp(&b.name));
        variants_sorted_by_value.sort_by(|a, b| a.value.cmp(&b.value));

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

            // These are not real enums (just specify a min time for basis of value)
            "date_time" => (),
            "local_date_time" => (),

            v => {
                enums.push(FittleEnum {
                    name: to_pascal_case(v),
                    module: v.to_owned(),
                    base_type: field_content_type(base_type).to_owned(),
                    variants,
                    variants_sorted_by_value,
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
        let mut fields = Vec::new();
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

            let is_array = match &field_data[MESSAGES_SHEET_ARRAY_COLUMN] {
                DataType::Empty => false,
                DataType::String(_) => true,
                v => panic!("unexpected type in message array column: {:?}", v),
            };

            let rust_type = rust_type(&field_type);

            let conversion_function = if is_array {
                format!("content.many().map(|vec| vec.into_iter().map(<{0}>::from).collect())", rust_type)
            } else {
                format!("content.one().map(<{0}>::from)", rust_type)
            };

            let rust_field_type = if is_array {
                format!("Vec<{0}>", rust_type)
            } else {
                rust_type
            };

            fields.push(FittleMessageField {
                name: field_name.to_owned(),
                number: field_number,
                rust_type: rust_field_type,
                conversion_function,
            })
        }

        let mut fields_sorted_by_number = fields.clone();
        fields.sort_by(|a, b| a.name.cmp(&b.name));
        fields_sorted_by_number.sort_by(|a, b| a.number.cmp(&b.number));

        messages.push(FittleMessage {
            name: to_pascal_case(message_name),
            module: message_name.clone(),
            fields,
            fields_sorted_by_number,
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

fn rust_type(field_type: &str) -> String {
    match field_type {
        // Simple base types
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

        // Specialized forms
        "date_time" => "crate::fields::DateTime".to_owned(),
        "local_date_time" => "crate::fields::LocalDateTime".to_owned(),

        // Everything else will be an enum
        v => format!("crate::profile::enums::{}", to_pascal_case(v)),
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
        Error { wrapped: format!("{}", err) }
    }
}

impl From<TemplateError> for Error {
    fn from(err: TemplateError) -> Self {
        Error { wrapped: format!("{}", err) }
    }
}
