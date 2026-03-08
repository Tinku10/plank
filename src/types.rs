use crate::serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum PlankType {
    Str,
    Int32,
    Int64,
    // Float64,
    Bool,
    // List(Box<PlankType>),
    // Struct(Vec<PlankField>),
}

#[derive(Debug, Clone)]
pub struct PlankField {
    name: String,
    field_type: PlankType,
}

#[derive(Debug, Clone)]
pub enum PlankData {
    Str(String),
    Int32(i32),
    Int64(i64),
    Bool(bool),
}

impl PlankField {
    pub fn new(name: &str, field_type: PlankType) -> Self {
        PlankField {
            name: String::from(name),
            field_type,
        }
    }
}

impl PlankField {
    pub fn field_type(&self) -> &PlankType {
        &self.field_type
    }
}

impl FromStr for PlankType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Str" => Ok(Self::Str),
            "Int32" => Ok(Self::Int32),
            "Int64" => Ok(Self::Int64),
            "Bool" => Ok(Self::Bool),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unable to infer type of {}", s),
            )),
        }
    }
}

impl fmt::Display for PlankType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str => write!(f, "Str"),
            Self::Int32 => write!(f, "Int32"),
            Self::Int64 => write!(f, "Int64"),
            Self::Bool => write!(f, "Bool"),
        }
    }
}

impl Serialize for PlankType {
    fn to_bytes(&self) -> Vec<u8> {
        let id: u8 = match self {
            Self::Str => 1,
            Self::Int32 => 2,
            Self::Int64 => 3,
            Self::Bool => 4,
        };
        id.to_le_bytes().to_vec()
    }
}

impl Deserialize for PlankType {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let id = bytes[0]
            .try_into()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u8"))?;
        match id {
            1 => Ok(Self::Str),
            2 => Ok(Self::Int32),
            3 => Ok(Self::Int64),
            4 => Ok(Self::Bool),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unknown type id {}", id),
            )),
        }
    }
}

impl Serialize for PlankField {
    fn to_bytes(&self) -> Vec<u8> {
        // Format: field_size field_name type_size type_name
        let mut v = Vec::new();
        let name_bytes = self.name.as_bytes();

        v.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        v.extend_from_slice(name_bytes);
        // Type ID will always be a u32
        // v.extend_from_slice(4u32.to_le_bytes());
        v.extend_from_slice(&self.field_type.to_bytes());

        v
    }
}

impl Deserialize for PlankField {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let size =
            u32::from_le_bytes(bytes[..4].try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u32")
            })?) as usize;

        let field_name = std::str::from_utf8(&bytes[4..4 + size as usize])
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("expected to read {} bytes", size),
                )
            })?
            .to_string();

        let field_type =
            PlankType::from_bytes(bytes[4 + size..8 + size].try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u32")
            })?)?;

        Ok(PlankField {
            name: field_name,
            field_type,
        })
    }
}

impl Serialize for PlankData {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            PlankData::Str(s) => {
                let mut v = Vec::new();
                let bytes = s.as_bytes();
                v.extend_from_slice(&PlankType::to_bytes(&PlankType::Str));
                v.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                v.extend_from_slice(bytes);
                v
            }
            PlankData::Int32(n) => {
                let mut v = Vec::new();
                v.extend_from_slice(&PlankType::to_bytes(&PlankType::Int32));
                v.extend_from_slice(&n.to_le_bytes());
                v
            }
            PlankData::Int64(n) => {
                let mut v = Vec::new();
                v.extend_from_slice(&PlankType::to_bytes(&PlankType::Int64));
                v.extend_from_slice(&n.to_le_bytes());
                v
            }
            PlankData::Bool(b) => {
                let mut v = Vec::new();
                v.extend_from_slice(&PlankType::to_bytes(&PlankType::Bool));
                v.extend_from_slice(&[*b as u8]);
                v
            }
        }
    }
}

impl Deserialize for PlankData {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let value_type = PlankType::from_bytes(bytes)?;
        match value_type {
            PlankType::Str => {
                let size = u32::from_le_bytes(bytes[1..5].try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u32")
                })?);
                let field_value = std::str::from_utf8(&bytes[5..5 + size as usize])
                    .map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("expected to read {} bytes", size),
                        )
                    })?
                    .to_string();
                Ok(PlankData::Str(field_value))
            }
            PlankType::Int32 => {
                let n = i32::from_le_bytes(bytes[1..5].try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u32")
                })?);
                Ok(PlankData::Int32(n))
            }
            PlankType::Int64 => {
                let n = i64::from_le_bytes(bytes[1..9].try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "expected u64")
                })?);
                Ok(PlankData::Int64(n))
            }
            PlankType::Bool => {
                let b = bytes[1].try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "expected bool")
                })?;
                Ok(PlankData::Bool(b))
            }
        }
    }
}
