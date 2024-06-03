use base64::{base64_encode, base64_decode};

fn main() {
    // Example data to encode
    let data = b"The quick brown fox jumps over the lazy dog";
    
    // Encode the data
    let encoded = base64_encode(data);
    println!("Encoded: {}", encoded);
    
    // Decode the data
    match base64_decode(&encoded) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8(decoded).expect("Invalid UTF-8 sequence");
            println!("Decoded: {}", decoded_str);
        },
        Err((err_msg, invalid_byte)) => {
            println!("Error: {} (invalid byte: {})", err_msg, invalid_byte);
        }
    }
}
