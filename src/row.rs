use crate::fragment::*;

#[derive(Debug)]
pub struct Row {
    name: String,
    values: Vec<String>,
}

impl Row {
    pub fn new(name: String, values: Vec<String>) -> Row {
        Row {
            name,
            values,
        }
    }

    pub fn builder_with_size(expected_values_size: usize) -> impl Fn(&[Fragment]) -> Result<Row, String> {
        return move |fragments: &[Fragment]| {
            let table_frags = fragments.iter()
                .filter(|f| Fragment::is_table_fragment(f))
                .collect::<Vec<&Fragment>>();
    
            let row_name_frags = fragments.iter()
                .filter(|f| f.is_row_name())
                .collect::<Vec<&Fragment>>();
    
            let row_value_frags = fragments.iter()
                .filter(|f| f.is_row_value())
                .collect::<Vec<&Fragment>>();
    
            let table_frags_count = table_frags.len();
            let row_name_frags_count = row_name_frags.len();
            let row_value_frags_count = row_value_frags.len();
    
            if table_frags_count > 0 {
                return Err(format!("Table fragment when trying to build a row. Fragments: {:?}", fragments));
            }
    
            if row_name_frags_count != 1 {
                return Err(format!("Expected one row name fragment, found multiple. Fragments: {:?}", fragments));
            }
    
            if expected_values_size != row_value_frags_count {
                return Err(format!("Expected `{}` row value fragments, `{}` fragment found. Fragments: {:?}", expected_values_size, row_value_frags_count, fragments));
            }
    
            if let Some(name) = row_name_frags.first() {
                let name = name.get_value();
                let values = row_value_frags.iter()
                    .map(|f| f.get_value())
                    .collect::<Vec<String>>();
    
                return Ok(Row::new(name, values));
            }
    
            Err(format!("Unable to build row from {:?}", fragments))
        }
    }
}