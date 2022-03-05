use crate::error::Error;
use crate::error::Error::*;
use crate::fragment::*;
use crate::fragment::Fragment::*;
use crate::row::*;
use crate::month::*;

#[derive(Debug)]
pub struct Table {
    name: String,
    month: Month,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String, month: Month, rows: Vec<Row>) -> Table {
        Table {
            name,
            month,
            rows,
        }
    }

    pub fn from(fragments: &[Fragment], rows: Vec<Row>) -> Result<Table, Error> {
        if let [TableName(name), TableHeader(month_str)] = fragments {
            let month = Month::from(month_str)
                .map_err(|e| 
                    ParseTableError(format!("Unable to build table. Fragments {:?}, cause: {:?}", fragments, e.to_string()))
                )?;

            return Ok(Table::new(name.clone(), month, rows));
        }

        Err(ParseTableError(format!("Unable to build table. Fragments {:?}", fragments)))
    }
}