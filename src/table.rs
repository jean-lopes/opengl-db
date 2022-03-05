use crate::fragment::*;
use crate::row::*;

#[derive(Debug)]
pub struct Table {
    name: String,
    header: Vec<String>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String, header: Vec<String>, rows: Vec<Row>) -> Table {
        Table {
            name,
            header,
            rows,
        }
    }

    pub fn builder_with_size(expected_header_size: usize) -> impl Fn(&[Fragment], Vec<Row>) -> Result<Table, String> {
        move |fragments: &[Fragment], rows: Vec<Row>| {
            let row_frags = fragments.iter()
                .filter(|f| Fragment::is_row_fragment(f))
                .collect::<Vec<&Fragment>>();

            let table_name_frags = fragments.iter()
                .filter(|f| f.is_table_name())
                .collect::<Vec<&Fragment>>();

            let table_header_frags = fragments.iter()
                .filter(|f| f.is_table_header())
                .collect::<Vec<&Fragment>>();

            let row_frags_count = row_frags.len();
            let table_name_frags_count = table_name_frags.len();
            let table_header_frags_count = table_header_frags.len();
    
            if row_frags_count > 0 {
                return Err(format!("Table fragment when trying to build a row. Fragments: {:?}", fragments));
            }
    
            if table_name_frags_count != 1 {
                return Err(format!("Expected one row name fragment, found multiple. Fragments: {:?}", fragments));
            }
    
            if expected_header_size != table_header_frags_count {
                return Err(format!("Expected `{}` table header fragments, `{}` fragment found. Fragments: {:?}", expected_header_size, table_header_frags_count, fragments));
            }

            if let Some(name) = table_name_frags.first() {
                let name = name.get_value();
                let header = table_header_frags.iter()
                    .map(|f| f.get_value())
                    .collect::<Vec<String>>();
    
                return Ok(Table::new(name, header, rows));
            }

            Err(format!("Unable to build table from {:?}", fragments))
        }
    }
}