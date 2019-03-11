use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FaveroProduct {
    AssiomaDuo,
    AssiomaUno,
    UnknownValue(u64),
}

impl From<FieldContent> for FaveroProduct {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                10 => FaveroProduct::AssiomaUno,
                12 => FaveroProduct::AssiomaDuo,
                n => FaveroProduct::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FaveroProduct to {:?}", field);
        }
    }
}
