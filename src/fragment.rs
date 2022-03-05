
#[derive(Debug)]
pub enum Fragment {
    TableName{ value: String },
    TableHeader{ value: String },
    RowName{ value: String },
    RowValue{ value: String },
}

impl Fragment {
    pub fn get_value(&self) -> String {
        match self {
            Fragment::TableName{value} => value.clone(),
            Fragment::TableHeader{value} => value.clone(),
            Fragment::RowName{value} => value.clone(),
            Fragment::RowValue{value} => value.clone(),
        }
    }


    pub fn is_row_name(&self) -> bool {
        match self {
            Fragment::RowName{..} => true,
            _ => false,
        }
    }

    pub fn is_row_value(&self) -> bool {
        match &self {
            Fragment::RowValue{..} => true,
            _ => false,
        }
    }

    pub fn is_row_fragment(&self) -> bool {
        self.is_row_name() | self.is_row_value()
    }

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