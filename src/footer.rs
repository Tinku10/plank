use crate::serde;
use std::fmt::Write;

pub struct Footer {
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
