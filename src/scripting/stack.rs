use binary_codec::{BitStreamReader, BitStreamWriter};

#[derive(Debug, Clone, PartialEq)]
pub enum StackData {
    Boolean(bool),
    Number(i128),
    Float(f64),
    Buffer(Vec<u8>),
    Byte(u8),
}

impl StackData {
    pub fn memory(&self) -> usize {
        match self {
            StackData::Boolean(_) => 1,
            StackData::Number(_) => 2,
            StackData::Float(_) => 3,
            StackData::Buffer(items) => items.len() * 2,
            StackData::Byte(_) => 2,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            StackData::Boolean(b) => Some(*b),
            StackData::Number(n) => match *n {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            },
            StackData::Buffer(b) => {
                if b.len() == 0 {
                    None
                } else {
                    StackData::Byte(b[0]).as_boolean()
                }
            }
            StackData::Byte(b) => match *b {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            },
            StackData::Float(f) => match *f {
                0.0 => Some(false),
                1.0 => Some(true),
                _ => None,
            }
        }
    }

    pub fn as_number(&self) -> Option<i128> {
        match self {
            StackData::Boolean(b) => {
                if *b {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            StackData::Number(n) => Some(*n),
            StackData::Buffer(b) => {
                if b.len() == 0 {
                    None
                } else {
                    // Try to read dynamic int
                    let mut reader = BitStreamReader::new(b);
                    reader.read_dyn_int().ok().map(|v| v as i128)
                }
            }
            StackData::Byte(b) => Some(*b as i128),
            StackData::Float(f) => Some(*f as i128),
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            StackData::Boolean(b) => {
                if *b {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            StackData::Number(n) => Some(*n as f64),
            StackData::Float(f) => Some(*f),
            StackData::Buffer(b) => {
                if b.len() < 8 {
                    None
                } else if b.len() == 8 {
                    Some(f64::from_be_bytes(b[0..8].try_into().unwrap()))
                } else {
                    self.as_number().map(|n| n as f64)
                }
            }
            StackData::Byte(b) => Some(*b as f64),
        }
    }

    pub fn as_byte(&self) -> Option<u8> {
        match self {
            StackData::Boolean(b) => {
                if *b {
                    Some(1)
                } else {
                    Some(0)
                }
            }
            StackData::Number(n) => (*n).try_into().ok(),
            StackData::Buffer(b) => {
                if b.len() == 0 {
                    None
                } else {
                    Some(b[0])
                }
            }
            StackData::Byte(b) => Some(*b),
            StackData::Float(f) => Some(f.round() as u8),
        }
    }

    pub fn as_buffer(&self) -> Option<Vec<u8>> {
        match self {
            StackData::Boolean(b) => {
                if *b {
                    Some(vec![0])
                } else {
                    Some(vec![1])
                }
            }
            StackData::Number(n) => {
                let mut buff = Vec::new();
                let mut writer = BitStreamWriter::new(&mut buff);
                writer.write_dyn_int(*n as u128);
                Some(buff)
            }
            StackData::Buffer(items) => Some(items.clone()),
            StackData::Byte(b) => Some(vec![*b]),
            StackData::Float(f) => {
                Some(f.to_be_bytes().to_vec().into())
            }
        }
    }
}
