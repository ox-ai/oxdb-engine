// use std::collections::HashMap;
use std::convert::TryInto;
// use std::io::{Cursor, Read, Write};

// Helper function to convert float to bytes
fn float_to_bytes(f: f64) -> [u8; 8] {
    f.to_bits().to_be_bytes()
}

// Helper function to convert bytes to float
fn bytes_to_float(bytes: &[u8]) -> f64 {
    f64::from_bits(u64::from_be_bytes(bytes.try_into().unwrap()))
}
pub fn encode(data: &serde_json::Value) -> Vec<u8> {
    match data {
        serde_json::Value::String(s) => {
            let mut bytes = vec![b's'];
            let length = s.len() as u32;
            bytes.extend(&length.to_be_bytes());
            bytes.extend(s.as_bytes());
            bytes
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                let mut bytes = vec![b'i'];
                bytes.extend(&i.to_be_bytes());
                bytes
            } else if let Some(f) = n.as_f64() {
                let mut bytes = vec![b'f'];
                bytes.extend(&float_to_bytes(f));
                bytes
            } else {
                panic!("Unsupported number type");
            }
        }
        serde_json::Value::Bool(b) => {
            let mut bytes = vec![b't']; // 't' for boolean
            bytes.push(if *b { 1 } else { 0 }); // 1 for true, 0 for false
            bytes
        }
        serde_json::Value::Array(arr) => {
            let mut bytes = vec![b'v'];
            let length = arr.len() as u32;
            bytes.extend(&length.to_be_bytes());
            for item in arr {
                bytes.extend(encode(item));
            }
            bytes
        }
        serde_json::Value::Object(obj) => {
            let mut bytes = vec![b'm'];
            let length = obj.len() as u32;
            bytes.extend(&length.to_be_bytes());
            for (key, value) in obj {
                bytes.extend(encode(&serde_json::Value::String(key.clone())));
                bytes.extend(encode(value));
            }
            bytes
        }
        _ => panic!("Unsupported data type: {:?}", data),
    }
}

pub fn decode(data: &[u8], pos: usize) -> (serde_json::Value, usize) {
    let data_type = data[pos] as char;
    let mut new_pos = pos + 1;

    match data_type {
        's' => {
            let length =
                u32::from_be_bytes(data[new_pos..new_pos + 4].try_into().unwrap()) as usize;
            new_pos += 4;
            let value = String::from_utf8(data[new_pos..new_pos + length].to_vec()).unwrap();
            new_pos += length;
            (serde_json::Value::String(value), new_pos)
        }
        'i' => {
            let value = i64::from_be_bytes(data[new_pos..new_pos + 8].try_into().unwrap());
            new_pos += 8;
            (serde_json::Value::Number(value.into()), new_pos)
        }
        'f' => {
            let value = bytes_to_float(&data[new_pos..new_pos + 8]);
            new_pos += 8;
            (
                serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap()),
                new_pos,
            )
        }
        't' => {
            let value = data[new_pos] == 1; // 1 for true, 0 for false
            new_pos += 1;
            (serde_json::Value::Bool(value), new_pos)
        }
        'v' => {
            let length =
                u32::from_be_bytes(data[new_pos..new_pos + 4].try_into().unwrap()) as usize;
            new_pos += 4;
            let mut values = Vec::new();
            for _ in 0..length {
                let (value, updated_pos) = decode(data, new_pos);
                new_pos = updated_pos;
                values.push(value);
            }
            (serde_json::Value::Array(values), new_pos)
        }
        'm' => {
            let length =
                u32::from_be_bytes(data[new_pos..new_pos + 4].try_into().unwrap()) as usize;
            new_pos += 4;
            let mut map = serde_json::Map::new();
            for _ in 0..length {
                let (key, updated_pos) = decode(data, new_pos);
                new_pos = updated_pos;
                let (value, updated_pos) = decode(data, new_pos);
                new_pos = updated_pos;
                if let serde_json::Value::String(key_str) = key {
                    map.insert(key_str, value);
                } else {
                    panic!("Invalid key type in map");
                }
            }
            (serde_json::Value::Object(map), new_pos)
        }
        _ => {
            // Catch-all case for unsupported types
            panic!(
                "Unsupported data type prefix: '{}', at position: {}",
                data_type, pos
            );
        }
    }
}

pub fn encode_n(totbytelen: usize) -> Vec<u8> {
    let datalen = totbytelen - 5;
    let mut deldata = vec![0u8; datalen]; // creates a byte array of 0s of length `datalen`

    let mut result = Vec::new();
    result.push(b'n'); // 'n' as the data type identifier
    result.extend(&(datalen as u32).to_be_bytes()); // add the length of the data
    result.append(&mut deldata); // append the zeroed data

    result
}

pub fn decode_n(
    data_bytes: &[u8],
    pos: usize,

) -> (i32, usize) {
    let bdsize_len = 5; // if length is None, `bdsize_len` is 5
    let length =  {
        u32::from_be_bytes(data_bytes[pos + 1..pos + 5].try_into().unwrap()) as usize
    };

    let value = 0; // the value for 'n' type is always 0

    (value, pos + bdsize_len + length)
}
#[test]
fn main() {
    // Example usage
    // Example usage
    let data = serde_json::json!({
        "name": "record",
        "alive": 10,
        "is_admin": true,
        "scores": [85, 92, 88],
        "details": {
            "notes": "md",
            "vlist": ["1",2]
        }
    });

    // Encode
    let encoded_data = encode(&data);
    println!("Encoded data: {:?},{}", encoded_data, encoded_data.len());

    // Decode
    // let (decoded_data, _)
    let decoded_data = decode(&encoded_data, 0);
    println!("Decoded data: {:?}", decoded_data);

    // Encode 'n' type
    let encoded_n = encode_n(20);
    println!("Encoded n: {:?}", encoded_n);

    // Decode 'n' type
    let (decoded_n, new_pos) = decode_n(&encoded_n, 0 );
    println!("Decoded n: {}, New Position: {}", decoded_n, new_pos);
}
