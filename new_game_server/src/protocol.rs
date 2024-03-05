// MADE BY: https://github.com/GDenisC

use std::fmt;

#[allow(dead_code)]
pub enum Message {
    Null,
    Bool(bool),
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Uint64(u64),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    Array(Vec<Message>),
    Object(Vec<(Message, Message)>),
}

impl fmt::Debug for Message {
    fn fmt(&self, x: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Null => write!(x, "Null"),
            Message::Bool(b) => write!(x, "Bool({})", b),
            Message::Uint8(u) => write!(x, "Uint8({})", u),
            Message::Int8(i) => write!(x, "Int8({})", i),
            Message::Uint16(u) => write!(x, "Uint16({})", u),
            Message::Int16(i) => write!(x, "Int16({})", i),
            Message::Uint32(u) => write!(x, "Uint32({})", u),
            Message::Int32(i) => write!(x, "Int32({})", i),
            Message::Uint64(u) => write!(x, "Uint64({})", u),
            Message::Int64(i) => write!(x, "Int64({})", i),
            Message::Float32(f) => write!(x, "Float32({})", f),
            Message::Float64(d) => write!(x, "Float64({})", d),
            Message::String(s) => write!(x, "String(\"{}\")", s),
            Message::Array(a) => write!(x, "Array({:?})", a),
            Message::Object(o) => write!(x, "Object({:?})", o),
        }
    }
}

#[allow(dead_code)]
/// ## Convert an index to bytes
/// ### {length of bytes}, {byte}, ...
///
/// Example 1 : 255 -> {1, 255}
///
/// Example 2: 65535 -> {2, 255, 255}
fn index_to_bytes(i: usize) -> Vec<u8> {
    if i < 0x80 {
        vec![1, i as u8]
    } else {
        vec![2, (i >> 8) as u8, i as u8]
    }
}

fn bytes_to_index(len: usize, b: &[u8]) -> usize {
    match len {
        1 => b[0] as usize,
        2 => ((b[0] as usize) << 8) + b[1] as usize,
        _ => panic!("Unsupported length"),
    }
}
#[allow(dead_code)]
impl Message {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Message::Null => vec![0x00],
            Message::Bool(b) => vec![if *b { 0x01 } else { 0x02 }],
            Message::Uint8(u) => vec![0x03, *u],
            Message::Int8(i) => vec![0x04, *i as u8],
            Message::Uint16(u) => vec![0x05, *u as u8, (*u >> 8) as u8],
            Message::Int16(i) => vec![0x06, *i as u8, (*i >> 8) as u8],
            Message::Uint32(u) => vec![
                0x07,
                *u as u8,
                (*u >> 8) as u8,
                (*u >> 16) as u8,
                (*u >> 24) as u8,
            ],
            Message::Int32(i) => vec![
                0x08,
                *i as u8,
                (*i >> 8) as u8,
                (*i >> 16) as u8,
                (*i >> 24) as u8,
            ],
            Message::Uint64(u) => vec![
                0x09,
                *u as u8,
                (*u >> 8) as u8,
                (*u >> 16) as u8,
                (*u >> 24) as u8,
                (*u >> 32) as u8,
                (*u >> 40) as u8,
                (*u >> 48) as u8,
                (*u >> 56) as u8,
            ],
            Message::Int64(i) => vec![
                0x0a,
                *i as u8,
                (*i >> 8) as u8,
                (*i >> 16) as u8,
                (*i >> 24) as u8,
                (*i >> 32) as u8,
                (*i >> 40) as u8,
                (*i >> 48) as u8,
                (*i >> 56) as u8,
            ],
            Message::Float32(f) => {
                let mut v = vec![0x0b];
                v.extend(f.to_le_bytes());
                v
            }
            Message::Float64(d) => {
                let mut v = vec![0x0c];
                v.extend(d.to_le_bytes());
                v
            }
            Message::String(s) => {
                let mut v = vec![0x0d];
                v.extend(index_to_bytes(s.len()));
                v.extend(s.as_bytes());
                v
            }
            Message::Array(a) => {
                let mut v = vec![0x0e];
                v.extend(index_to_bytes(a.len()));
                for m in a {
                    v.extend(m.encode());
                }
                v
            }
            Message::Object(o) => {
                let mut v = vec![0x0f];
                v.extend(index_to_bytes(o.len()));
                for (k, m) in o {
                    v.extend(k.encode());
                    v.extend(m.encode());
                }
                v
            }
        }
    }

    pub fn decode(buf: &[u8]) -> Self {
        if buf.is_empty() {
            return Message::Null;
        }
        let message_type = buf[0] as usize;
        match message_type {
            0x00 => Message::Null,
            0x01 => Message::Bool(true),
            0x02 => Message::Bool(false),
            0x03 => Message::Uint8(buf[1]),
            0x04 => Message::Int8(buf[1] as i8),
            0x05 => Message::Uint16((buf[1] as u16) + ((buf[2] as u16) << 8) as u16),
            0x06 => Message::Int16((buf[1] as i16) + ((buf[2] as u16) << 8) as i16),
            0x07 => Message::Uint32(
                (buf[1] as u32)
                    + ((buf[2] as u32) << 8) as u32
                    + ((buf[3] as u32) << 16) as u32
                    + ((buf[4] as u32) << 24) as u32,
            ),
            0x08 => Message::Int32(
                (buf[1] as i32)
                    + ((buf[2] as i32) << 8) as i32
                    + ((buf[3] as i32) << 16) as i32
                    + ((buf[4] as i32) << 24) as i32,
            ),
            0x09 => Message::Uint64(
                (buf[1] as u64)
                    + ((buf[2] as u64) << 8) as u64
                    + ((buf[3] as u64) << 16) as u64
                    + ((buf[4] as u64) << 24) as u64
                    + ((buf[5] as u64) << 32) as u64
                    + ((buf[6] as u64) << 48) as u64
                    + ((buf[7] as u64) << 56) as u64,
            ),
            0x0a => Message::Int64(i64::from_le_bytes(buf[1..9].try_into().unwrap())),
            0x0b => Message::Float32(f32::from_le_bytes(buf[1..5].try_into().unwrap())),
            0x0c => Message::Float64(f64::from_le_bytes(buf[1..9].try_into().unwrap())),
            0x0d => {
                let mut offset: usize = 0;
                let index_len = buf[1] as usize;
                offset += 1;
                offset += index_len;
                let length = bytes_to_index(index_len, &buf[offset..(offset + index_len)]);
                offset += 1;
                let mut s = String::new();
                s.push_str(String::from_utf8_lossy(&buf[offset..(offset + length)]).as_ref());
                Message::String(s)
            }
            0x0e => {
                let mut offset: usize = 0;
                let index_len = buf[1] as usize;
                offset += 1;
                offset += index_len;
                let length = bytes_to_index(index_len, &buf[offset..(offset + index_len)]);
                let mut a = Vec::new();
                let mut index = offset + 1;
                for _ in 0..length {
                    let message = Message::decode(&buf[index..]);
                    index += message.length();
                    a.push(message);
                }
                Message::Array(a)
            }
            0x0f => {
                let mut offset: usize = 0;
                let index_len = buf[1] as usize;
                offset += 1;
                offset += index_len;
                let length = bytes_to_index(index_len, &buf[offset..(offset + index_len)]);
                let mut o = Vec::new();
                let mut index = offset + 1;
                for _ in 0..length {
                    let key = Message::decode(&buf[index..]);
                    //println!("key {}+{} {:?}", index, key.length(), &buf[index..]);
                    index += key.length();
                    let value = Message::decode(&buf[index..]);
                    //println!("value {}+{} {:?}", index, value.length(), &buf[index..]);
                    index += value.length();
                    o.push((key, value));
                }
                Message::Object(o)
            }
            _ => panic!("{}", format!("Message not found for {message_type}")),
        }
    }

    pub fn length(&self) -> usize {
        match self {
            Message::Null => 1,
            Message::Bool(_) => 1,
            Message::Uint8(_) => 1 + 1,
            Message::Int8(_) => 1 + 1,
            Message::Uint16(_) => 1 + 2,
            Message::Int16(_) => 1 + 2,
            Message::Uint32(_) => 1 + 4,
            Message::Int32(_) => 1 + 4,
            Message::Uint64(_) => 1 + 8,
            Message::Int64(_) => 1 + 8,
            Message::Float32(_) => 1 + 4,
            Message::Float64(_) => 1 + 8,
            Message::String(s) => 1 + 1 + if s.len() > 255 { 2 } else { 1 } + s.chars().count(),
            Message::Array(a) => {
                1 + 1
                    + if a.len() > 255 { 2 } else { 1 }
                    + a.iter().map(|x| x.length()).sum::<usize>()
            }
            Message::Object(o) => {
                1 + 1
                    + if o.len() > 255 { 2 } else { 1 }
                    + o.iter()
                        .map(|(k, v)| k.length() + v.length())
                        .sum::<usize>()
            }
        }
    }
}
