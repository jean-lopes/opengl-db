use std::{fmt};

use crate::fragment::Fragment;
use crate::fragment::Fragment::*;
use crate::Error;
use crate::Error::*;

#[derive(Debug)]
pub struct Row {
    name: String,
    value: f32,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Row {{ name: {}, value: {} }}", self.name, self.value)
    }
}

impl Row {
    pub fn new(name: String, value: f32) -> Row {
        Row {
            name,
            value,
        }
    }

    pub fn from(fragments: &[Fragment]) -> Result<Row, Error> {
        if let [RowName(name), RowValue(value_str)] = fragments {
            let value = value_str.trim()
                .replace("-", "0.00")
                .replace("%", "")
                .parse::<f32>()
                .map_err(|e| 
                    ParseRowError(format!("Unable to build row from {:?}, cause: {}", fragments, e.to_string()))
                )?;

            return Ok(Row::new(name.clone(), value));
        }

        Err(ParseRowError(format!("Unable to build row from {:?}", fragments)))
    }
}