use crate::serde;
use std::fmt::Write;

pub(crate) struct Footer {
    schema: Vec<(String, String)>,
    offsets: Vec<u32>,
    row_count: u32,
    col_count: u32,
}

impl Footer {
    pub fn new(
        schema: Vec<(String, String)>,
        offsets: Vec<u32>,
        row_count: u32,
        col_count: u32,
    ) -> Self {
        Footer {
            schema,
            offsets,
            row_count,
            col_count,
        }
    }

    pub fn schema(&self) -> &Vec<(String, String)> {
        &self.schema
    }

    pub fn offsets(&self) -> &Vec<u32> {
        // TODO: Provide a better way to request limited offsets
        &self.offsets
    }

    pub fn row_count(&self) -> u32 {
        self.row_count
    }

    pub fn col_count(&self) -> u32 {
        self.col_count
    }
}

impl serde::Serialize for Footer {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str("!SCHEMA=");
        for (k, v) in &self.schema {
            write!(s, "{}:{}", k, v);
            s.push(',')
        }
        s.push_str("\n");

        s.push_str("!OFFSETS=");
        for offset in &self.offsets {
            s.push_str(&offset.to_string());
            s.push(',')
        }
        s.push_str("\n");

        write!(s, "!RCOUNT={}\n", self.row_count.to_string());
        write!(s, "!CCOUNT={}\n", self.col_count.to_string());

        // s.push_str("!FOOTER=");

        s
    }
}
