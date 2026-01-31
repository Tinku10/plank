pub mod column;

use crate::serde;
use column::Column;

pub struct RowGroup {
    // id: u32,
    columns: Vec<Column>,
    offsets: Vec<u32>,
}

impl RowGroup {
    pub fn new(columns: Vec<Column>, offsets: Vec<u32>) -> Self {
        RowGroup {columns, offsets}
    }
}

impl serde::Serialize for RowGroup {
    fn to_string(&self) -> String {
        let mut v = Vec::new();
        for col in &self.columns {
            v.push(col.to_string());
        }

        v.push("\n".to_string());
        v.join("\n")
    }
}
