use crate::serde;

pub(crate) struct Column {
    // id: u32,
    records: Vec<String>,
}

impl Column {
    pub fn new(records: Vec<String>) -> Self {
        Column {records}
    }
}

impl serde::Serialize for Column {
    fn to_string(&self) -> String {
        self.records
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
