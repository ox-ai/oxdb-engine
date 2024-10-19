use oxdb_engine::oxdbin::basebin;


fn main() {
    // Example usage
    let data = serde_json::json!([[1,2,3,4],
        {
        "name": "record",
        "alive": 10,
        "is_admin": true,
        "scores": [85, 92, 88],
        "details": {
            "notes": "md",
            "vlist": ["1",2]
        }
    }]);


    // Encode
    let encoded_data = basebin::encode(&data);
    println!("Encoded data: {:?},{}", encoded_data, encoded_data.len());

    // Decode
    // let (decoded_data, _)
    let decoded_data = basebin::decode(&encoded_data, 0);
    println!("Decoded data: {:?}", decoded_data);

    // Encode 'n' type
    let encoded_n = basebin::encode_n(20);
    println!("Encoded n: {:?}", encoded_n);

    // Decode 'n' type
    let (decoded_n, new_pos) = basebin::decode_n(&encoded_n, 0 );
    println!("Decoded n: {}, New Position: {}", decoded_n, new_pos);
}









