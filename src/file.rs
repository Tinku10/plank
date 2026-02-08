pub(crate) mod footer;
pub(crate) mod rowgroup;

use footer::Footer;
use rowgroup::row::Row;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct SF2Meta {
    footer: Footer,
}

pub struct SF2Reader {
    file: BufReader<File>,
    meta: SF2Meta,
}

pub struct SF2Iterator<'a> {
    reader: &'a mut SF2Reader,
    offsets: Vec<u32>,
    row_count: u32,
    row_group: usize,
}

impl SF2Meta {
    pub fn schema(&self) -> &Vec<(String, String)> {
        self.footer.schema()
    }

    pub fn col_count(&self) -> u32 {
        self.footer.col_count()
    }

    pub fn row_count(&self) -> u32 {
        self.footer.row_count()
    }
}

impl SF2Reader {
    fn parse_schema(line: &str) -> Vec<(String, String)> {
        line.trim()
            .split(',')
            .filter_map(|item| {
                let mut it = item.split(':');
                match (it.next(), it.next()) {
                    (Some(col), Some(ty)) => Some((col.to_string(), ty.to_string())),
                    _ => None,
                }
            })
            .collect()
    }

    fn parse_offsets(line: &str) -> Vec<u32> {
        line.trim()
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    }

    pub fn open<P: AsRef<Path>>(file_path: P) -> std::io::Result<Self> {
        let mut f = File::open(file_path)?;
        // TODO: Check if file ends in newline

        // Footer offset for u32 including one byte for newline
        f.seek(SeekFrom::End(-5))?;

        let mut footer_offset = String::new();
        f.read_to_string(&mut footer_offset)?;
        let footer_offset = footer_offset.trim().parse::<u32>().unwrap();

        f.seek(SeekFrom::Start(footer_offset as u64));

        let mut br = BufReader::new(f);

        let mut s = String::new();

        br.read_line(&mut s);
        let schema = s
            .strip_prefix("!SCHEMA=")
            .ok_or_else(|| std::io::ErrorKind::InvalidData)?
            .to_string();

        let schema = Self::parse_schema(&schema);
        s.clear();

        br.read_line(&mut s);
        let offsets = s
            .strip_prefix("!OFFSETS=")
            .ok_or_else(|| std::io::ErrorKind::InvalidData)?
            .to_string();

        let offsets = Self::parse_offsets(&offsets);
        s.clear();

        br.read_line(&mut s);

        let row_count = s
            .strip_prefix("!RCOUNT=")
            .ok_or_else(|| std::io::ErrorKind::InvalidData)?
            .trim()
            .parse::<u32>()
            .unwrap();
        s.clear();

        br.read_line(&mut s);
        let col_count = s
            .strip_prefix("!CCOUNT=")
            .ok_or_else(|| std::io::ErrorKind::InvalidData)?
            .trim()
            .parse::<u32>()
            .unwrap();

        let footer = Footer::new(schema, offsets, row_count, col_count);
        let meta = SF2Meta { footer };

        Ok(Self { file: br, meta })
    }

    pub fn get_schema(&self) -> &Vec<(String, String)> {
        &self.meta.footer.schema()
    }

    pub fn head(&mut self, rows: Option<u32>) -> Option<Vec<Vec<String>>> {
        // Read up to first two rows
        let col_count = self.meta.footer.col_count() as usize;
        let mut offsets = self.meta.footer.offsets().clone();

        let mut rg = 0;

        let row_count = match rows {
            Some(x) => std::cmp::min(x, self.meta.row_count()),
            None => 2,
        } as usize;

        // TODO: Find a way to cast the data to the correct types
        // This is not the right place to cast it, but the stored type should be created in a way
        // to support it
        let mut result = vec![Vec::new(); row_count];

        for j in 0..row_count {
            for i in 0..col_count {
                let idx: usize = (rg * col_count + i) as usize;
                let offset = offsets[idx];
                self.file.seek(SeekFrom::Start(offset.into()));

                let mut item = Vec::new();
                let bytes = self.file.read_until(b',', &mut item).ok()?;
                item = item.strip_suffix(&[b',']).unwrap_or(&item).to_vec();

                result[j].push(String::from_utf8(item).ok()?);

                offsets[idx] += bytes as u32;
            }

            if self.file.peek(1).unwrap() == b"\n" {
                rg += 1;
            }
        }
        Some(result)
    }

    pub fn iter(&mut self) -> SF2Iterator {
        let offsets = self.meta.footer.offsets().clone();
        SF2Iterator {
            reader: self,
            offsets,
            row_count: 0,
            row_group: 0,
        }
    }
}

impl<'a> Iterator for SF2Iterator<'a> {
    type Item = std::io::Result<Vec<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let br = &mut self.reader.file;
        let meta = &self.reader.meta;
        let col_count = meta.footer.col_count() as usize;

        if self.row_count == meta.footer.row_count() {
            return None;
        }

        // TODO: Find a way to cast the data to the correct types
        // This is not the right place to cast it, but the stored type should be created in a way
        // to support it
        let mut result = Vec::with_capacity(col_count);

        for i in 0..col_count {
            let idx: usize = (self.row_group * col_count + i) as usize;
            let offset = self.offsets[idx];
            br.seek(SeekFrom::Start(offset.into()));

            let mut item = Vec::new();
            let bytes = br.read_until(b',', &mut item).ok()?;
            item = item.strip_suffix(&[b',']).unwrap_or(&item).to_vec();

            result.push(String::from_utf8(item).ok()?);

            self.offsets[idx] += bytes as u32;
            // offsets[idx] += bytes as u32;
        }

        if br.peek(1).unwrap() == b"\n" {
            self.row_group += 1;
        }

        self.row_count += 1;

        Some(Ok(result))
    }
}
