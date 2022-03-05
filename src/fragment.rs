
#[derive(Debug)]
pub enum Fragment {
    TableName(String),
    TableHeader(String),
    RowName(String),
    RowValue(String),
}

impl Fragment {
    pub fn is_table_name(&self) -> bool {
        match self {
            Fragment::TableName{..} => true,
            _ => false,
        }
    }

    pub fn is_table_header(&self) -> bool {
        match &self {
            Fragment::TableHeader{..} => true,
            _ => false,
        }
    }

    pub fn is_table_fragment(&self) -> bool {
        self.is_table_name() | self.is_table_header()
    }
}