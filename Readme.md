# Base64 Encoder and Decoder Rust

A Rust implementation of a Base64 encoder and decoder. This project includes functions to encode binary data into a Base64 string and decode a Base64 string back into binary data.

## Table of Contents

- [Introduction](#introduction)
- [Usage](#usage)
  - [Encoding](#encoding)
  - [Decoding](#decoding)
- [Examples](#examples)
- [Installation](#installation)
- [Code Explaination](#code-explaination)
- [Running Tests](#running-tests)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Base64 encoding is a method to convert binary data into an ASCII string format using 64 printable characters (A-Z, a-z, 0-9, +, and /). This encoding scheme is widely used for transmitting binary data over media designed to handle text.

- This project provides a simple and efficient implementation of Base64 encoding and decoding in Rust.

## Usage

### Encoding

To encode data using Base64, use the `base64_encode` function provided by the library. It takes a byte slice and returns a Base64 encoded string.

### Decoding

To decode a Base64 encoded string, use the `base64_decode` function. It takes a Base64 encoded string and returns a byte vector (if decoding is successful) or an error.

## Examples

### Encoding Example

Here is an example of encoding data to a Base64 string:

```rust
use base64::base64_encode;

fn main() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let encoded = base64_encode(data);
    println!("Encoded: {}", encoded);
}
```

### Decoding Example

Here is an example of decoding a Base64 string back to its original binary data:

```rust
use base64::base64_decode;

fn main() {
    let encoded = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==";
    
    match base64_decode(encoded) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8(decoded).expect("Invalid UTF-8 sequence");
            println!("Decoded: {}", decoded_str);
        },
        Err((err_msg, invalid_byte)) => {
            println!("Error: {} (invalid byte: {})", err_msg, invalid_byte);
        }
    }
}
```

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
base64 = { path = "path/to/your/library" }
```

Then, import the library in your `main.rs` or `lib.rs`:

```rust
use base64::{base64_encode, base64_decode};
```

# Code Explaination
Certainly! Let's break down the working of the code and the `base64_encode` and `base64_decode` functions in detail.

### Working of the Code

**Base64 Encoding and Decoding Overview**:
- Base64 encoding converts binary data into a textual representation using 64 ASCII characters. Each Base64 character represents 6 bits of the original binary data.
- Decoding is the reverse process, converting the Base64 text back into binary data.

### Constants

1. **CHARSET**:
   ```rust
   const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
   ```
   This defines the 64 characters used in Base64 encoding.

2. **PADDING**:
   ```rust
   const PADDING: char = '=';
   ```
   This character is used for padding the Base64 encoded string when the input data is not a multiple of 3 bytes.

### Helper Function

**collect_six_bits**:
```rust
fn collect_six_bits(from: (u8, u8), offset: u8) -> u8 {
    let combined: u16 = ((from.0 as u16) << 8) | (from.1 as u16);
    ((combined & (0b1111110000000000u16 >> offset)) >> (10 - offset)) as u8
}
```
- **Parameters**: Takes a tuple of two bytes (`from`) and an offset (`offset`).
- **Combines** the two bytes into a single 16-bit integer.
- **Masks** and extracts 6 bits from the combined value based on the offset.
- **Returns**: A single byte (u8) containing the 6 bits extracted.

### Encoding Function

**base64_encode**:
```rust
pub fn base64_encode(data: &[u8]) -> String {
    let mut encoded_string = String::new();
    let mut bits_encoded = 0usize;

    let padding_needed = ((6 - (data.len() * 8) % 6) / 2) % 3;
    loop {
        let lower_byte_index_to_encode = bits_encoded / 8usize;
        if lower_byte_index_to_encode == data.len() {
            break;
        };

        let lower_byte_to_encode = data[lower_byte_index_to_encode];
        let upper_byte_to_code = if (lower_byte_index_to_encode + 1) == data.len() {
            0u8
        } else {
            data[lower_byte_index_to_encode + 1]
        };

        let bytes_to_encode = (lower_byte_to_encode, upper_byte_to_code);
        let offset: u8 = (bits_encoded % 8) as u8;
        encoded_string.push(CHARSET[collect_six_bits(bytes_to_encode, offset) as usize] as char);

        bits_encoded += 6;
    }

    for _ in 0..padding_needed {
        encoded_string.push(PADDING);
    }

    encoded_string
}
```
- **Input**: Takes a byte slice (`&[u8]`) of the data to be encoded.
- **Initializes** an empty string `encoded_string` to hold the resulting Base64 encoded string.
- **Calculates** the necessary padding based on the length of the input data.
- **Loop**:
  - Determines the index of the current byte to encode based on `bits_encoded`.
  - If all bytes are processed, the loop breaks.
  - Retrieves the current byte and the next byte (if available) to form a 16-bit combined value.
  - Calls `collect_six_bits` to get 6 bits from the combined value.
  - Maps the 6 bits to a Base64 character using `CHARSET` and appends it to `encoded_string`.
  - Increments `bits_encoded` by 6.
- **Adds** padding characters if needed.
- **Returns** the encoded string.

### Decoding Function

**base64_decode**:
```rust
pub fn base64_decode(data: &str) -> Result<Vec<u8>, (&str, u8)> {
    let mut collected_bits = 0;
    let mut byte_buffer = 0u16;
    let mut databytes = data.bytes();
    let mut outputbytes = Vec::<u8>::new();

    'decodeloop: loop {
        while collected_bits < 8 {
            if let Some(nextbyte) = databytes.next() {
                if let Some(idx) = CHARSET.iter().position(|&x| x == nextbyte) {
                    byte_buffer |= ((idx & 0b00111111) as u16) << (10 - collected_bits);
                    collected_bits += 6;
                } else if nextbyte == (PADDING as u8) {
                    collected_bits -= 2;
                } else {
                    return Err(("Failed to decode base64: Expected byte from charset, found invalid byte.", nextbyte));
                }
            } else {
                break 'decodeloop;
            }
        }
        outputbytes.push(((0b1111111100000000 & byte_buffer) >> 8) as u8);
        byte_buffer &= 0b0000000011111111;
        byte_buffer <<= 8;
        collected_bits -= 8;
    }

    if collected_bits != 0 {
        return Err(("Failed to decode base64: Invalid padding.", collected_bits));
    }

    Ok(outputbytes)
}
```
- **Input**: Takes a Base64 encoded string (`&str`).
- **Initializes**:
  - `collected_bits` to keep track of bits collected.
  - `byte_buffer` to store bits as they are collected.
  - `databytes` as an iterator over the bytes of the input string.
  - `outputbytes` as a vector to store the decoded bytes.
- **Loop**:
  - Collects bits until at least 8 bits are available.
  - Retrieves the next byte from the iterator.
  - Checks if the byte is in the `CHARSET` and gets its index, otherwise checks for padding.
  - Updates `byte_buffer` with the 6 bits from the current byte.
  - Adds the byte to the `outputbytes` when at least 8 bits are available.
  - Adjusts `byte_buffer` and `collected_bits` for the next iteration.
- **Handles padding** by reducing `collected_bits` when a padding character is encountered.
- **Returns** the decoded byte vector or an error if there is an issue with the input string (invalid characters or incorrect padding).

### Testing

**Tests**:
- **`pregenerated_random_bytes_encode`**: Tests the encoding function with predefined byte arrays and their expected Base64 encoded strings.
- **`pregenerated_random_bytes_decode`**: Tests the decoding function with predefined Base64 encoded strings and their expected byte arrays.
- **`encode_decode`**: Tests the encoding and then decoding of predefined byte arrays to ensure the output matches the original input.
- **`decode_encode`**: Tests the decoding and then encoding of predefined Base64 encoded strings to ensure the output matches the original input.

These tests ensure that the encoding and decoding functions work correctly for a variety of inputs, including edge cases and random data.

## Running Tests

To run the provided tests for the encoder and decoder, use the following command:

```sh
cargo test
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue. If you'd like to contribute code, please fork the repository and create a pull request.

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes.
4. Push to your branch.
5. Create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
