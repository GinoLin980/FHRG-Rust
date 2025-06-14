use std::convert::{TryFrom, TryInto};
use std::collections::HashMap;
use std::fmt;
use crate::data_schema;

pub fn decode_data(data: &[u8]) -> HashMap<String, DataType> {
    let mut return_map: HashMap<String, DataType> = HashMap::new();
    let data_types = &*data_schema::DATA_SCHEMA;
    let mut offset = 0;

    for (key, value) in data_types {
        let jump = match DataType::size_from_str(value) {
            Some(size) => size,
            None => continue,
        };

        if offset + jump > data.len() {
            eprintln!("Not enough data for field {}", key);
            continue;
        }

        let current_data = &data[offset..offset + jump];
        offset += jump;

        let decoded = match *value {
            "s32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::Int(i32::from_le_bytes(bytes))
            },
            "u32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::UInt(u32::from_le_bytes(bytes))
            },
            "f32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::Float(f32:: from_le_bytes(bytes))
            },
            "u16" => {
                let bytes: [u8; 2] = current_data.try_into().unwrap();
                DataType::UShort(u16:: from_le_bytes(bytes))
            },
            "u8" => {
                let bytes: [u8; 1] = current_data.try_into().unwrap();
                DataType::UByte(bytes[0])
            },
            "s8" => {
                let bytes: [u8; 1] = current_data.try_into().unwrap();
                DataType::SByte(i8::from_le_bytes(bytes))
            },
            _ => {
                // eprintln!("Unknown data => {}: {}", key, value);
                DataType::Int(0)
            }
        };
        return_map.insert(key.to_string(), decoded);
    }
    return_map
}


// ['"s32",', '"u32",', '"f32",', '"hzn",', '"u16",', '"u8",', '"s8",']
#[derive(Debug, Clone, Copy)] // Clone and Copy are useful for simple data types
pub enum DataType {
    Int(i32),
    UInt(u32),
    Float(f32),
    UShort(u16),
    UByte(u8),
    SByte(i8),
}

impl DataType {
    pub fn data_size(&self) -> usize {
        match self {
            DataType::Int(_) => std::mem::size_of::<i32>(),   // 4
            DataType::UInt(_) => std::mem::size_of::<u32>(),  // 4
            DataType::Float(_) => std::mem::size_of::<f32>(), // 4
            DataType::UShort(_) => std::mem::size_of::<u16>(), // 2
            DataType::UByte(_) => std::mem::size_of::<u8>(),  // 1
            DataType::SByte(_) => std::mem::size_of::<i8>(),  // 1
        }
    }

    pub fn size_from_str(name: &str) -> Option<usize> {
        match name {
            "s32" => Some(4),
            "u32" => Some(4),
            "f32" => Some(4),
            "hzn" => Some(12),
            "u16" => Some(2),
            "u8" => Some(1),
            "s8" => Some(1),
            _ => None,
        }
    }
}


// A more descriptive error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionError {
    description: String,
}

// Implement Display for better error messages
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

// Macro to reduce boilerplate for the implementations
macro_rules! impl_try_from_data_type {
    ($target_type:ty, $variant:ident) => {
        // Implementation for owned DataType -> T
        impl TryFrom<DataType> for $target_type {
            type Error = ConversionError;
            fn try_from(value: DataType) -> Result<Self, Self::Error> {
                match value {
                    DataType::$variant(val) => Ok(val),
                    _ => Err(ConversionError {
                        description: format!("Failed to convert {:?} to {}", value, stringify!($target_type)),
                    }),
                }
            }
        }

        // Implementation for borrowed &DataType -> T
        // This is crucial for working with collections like HashMap
        impl TryFrom<&DataType> for $target_type {
            type Error = ConversionError;
            fn try_from(value: &DataType) -> Result<Self, Self::Error> {
                match *value {
                    DataType::$variant(val) => Ok(val),
                    _ => Err(ConversionError {
                        description: format!("Failed to convert {:?} to {}", value, stringify!($target_type)),
                    }),
                }
            }
        }
    };
}

// Use the macro to generate implementations for all variants
impl_try_from_data_type!(i32, Int);
impl_try_from_data_type!(u32, UInt);
impl_try_from_data_type!(f32, Float);
impl_try_from_data_type!(u16, UShort);
impl_try_from_data_type!(u8, UByte);
impl_try_from_data_type!(i8, SByte);