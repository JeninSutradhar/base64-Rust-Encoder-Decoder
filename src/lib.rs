/// A Rust implementation of Base64 Encoder and Decoder

/// The charset and Padding used for encoding and decoding

// This defines the 64 characters used in Base64 encoding.
const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// This character is used for padding the Base64 encoded string
// when the input data is not a multiple of 3 bytes.
const PADDING: char = '=';


/// Combines two provided bytes into a u16 and collects 6 bits from it using an AND mask
///
/// Example:
///     Bytes: X and Y
///     (bits of those bytes will be signified using the names of their byte)
///     Offset: 4
///
/// 'combined' = 0bXXXXXXXXYYYYYYYY
/// AND mask:
///     0b1111110000000000 >> offset (4) = 0b0000111111000000
/// `combined` with mask applied:
///     0b0000XXYYYY000000
/// Shift the value right by (16 bit number) - (6 bit mask) - (4 offset) = 6:
/// 0b0000000000XXYYYY
/// And then turn it into a u8:
///     0b00XXYYYY (Return value)
///
/// Parameters:
/// - `from`: Takes a tuple of two bytes.
/// - `offset`: The offset value.
///
/// Combines the two bytes into a single 16-bit integer.
/// Masks and extracts 6 bits from the combined value based on the offset.
/// Returns: A single byte (u8) containing the 6 bits extracted.
fn collect_six_bits(from: (u8, u8), offset: u8) -> u8 {
    let combined: u16 = ((from.0 as u16) << 8) | (from.1 as u16);
    ((combined & (0b1111110000000000u16 >> offset)) >> (10 - offset)) as u8
}

/// Base64 encoding converts binary data into a textual representation
/// using 64 ASCII characters. Each Base64 character represents 6 bits 
/// of the original binary data.
///
/// Parameters:
/// - `data`: A byte slice (`&[u8]`) of the data to be encoded.
///
/// Returns: A Base64 encoded string.
pub fn base64_encode(data: &[u8]) -> String {
    let mut encoded_string = String::new();
    let mut bits_encoded = 0usize;

    // Using modulo twice to prevent an underflow   
    let padding_needed = ((6 - (data.len() * 8) % 6) / 2) % 3;
    loop {
        // Integer division
        let lower_byte_index_to_encode = bits_encoded / 8usize;
        if lower_byte_index_to_encode == data.len() {
            break;
        };

        let lower_byte_to_encode = data[lower_byte_index_to_encode];
        let upper_byte_to_encode = if (lower_byte_index_to_encode + 1) == data.len() {
            0u8
        } else {
            data[lower_byte_index_to_encode + 1]
        };

        let bytes_to_encode = (lower_byte_to_encode, upper_byte_to_encode);
        let offset: u8 = (bits_encoded % 8) as u8;
        encoded_string.push(CHARSET[collect_six_bits(bytes_to_encode, offset) as usize] as char);

        bits_encoded += 6;
    }

    for _ in 0..padding_needed {
        encoded_string.push(PADDING);
    }

    encoded_string
}

/// Base64 decoding converts a Base64 encoded string back into binary data.
///
/// Parameters:
/// - `data`: A Base64 encoded string.
///
/// Returns: A `Result` which is:
/// - `Ok(Vec<u8>)` containing the decoded byte vector on success.
/// - `Err((&str, u8))` with an error message and invalid byte on failure.
pub fn base64_decode(data: &str) -> Result<Vec<u8>, (&str, u8)> {
    let mut collected_bits = 0;
    let mut byte_buffer = 0u16;
    let mut databytes = data.bytes();
    let mut outputbytes = Vec::<u8>::new();

    'decodeloop: loop {
        while collected_bits < 8 {
            if let Some(nextbyte) = databytes.next() {
                // Finds the first occurrence of the latest byte
                if let Some(idx) = CHARSET.iter().position(|&x| x == nextbyte) {
                    byte_buffer |= ((idx & 0b00111111) as u16) << (10 - collected_bits);
                    collected_bits += 6;
                } else if nextbyte == (PADDING as u8) {
                    collected_bits -= 2; // Padding only comes at the end so this works
                } else {
                    return Err((
                        "Failed to decode base64: Expected byte from charset, found invalid byte.",
                        nextbyte,
                    ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pregenerated_random_bytes_encode() {
        macro_rules! test_encode {
            ($left: expr, $right: expr) => {
                assert_eq!(base64_encode(&$left.to_vec()), $right);
            };
        }
        test_encode!(
            b"\xd31\xc9\x87D\xfe\xaa\xb3\xff\xef\x8c\x0eoD",
            "0zHJh0T+qrP/74wOb0Q="
        );
        test_encode!(
            b"\x9f\x0e8\xbc\xf5\xd0-\xb4.\xd4\xf0?\x8f\xe7\t{.\xff/6\xcbTY!\xae9\x82",
            "nw44vPXQLbQu1PA/j+cJey7/LzbLVFkhrjmC"
        );
        test_encode!(b"\x7f3\x15\x1a\xd3\xf91\x9bS\xa44=", "fzMVGtP5MZtTpDQ9");
        test_encode!(
            b"7:\xf5\xd1[\xbfV/P\x18\x03\x00\xdc\xcd\xa1\xecG",
            "Nzr10Vu/Vi9QGAMA3M2h7Ec="
        );
        test_encode!(
            b"\xc3\xc9\x18={\xc4\x08\x97wN\xda\x81\x84?\x94\xe6\x9e",
            "w8kYPXvECJd3TtqBhD+U5p4="
        );
        test_encode!(
            b"\x8cJ\xf8e\x13\r\x8fw\xa8\xe6G\xce\x93c*\xe7M\xb6\xd7",
            "jEr4ZRMNj3eo5kfOk2Mq50221w=="
        );
        test_encode!(
            b"\xde\xc4~\xb2}\xb1\x14F.~\xa1z|s\x90\x8dd\x9b\x04\x81\xf2\x92{",
            "3sR+sn2xFEYufqF6fHOQjWSbBIHykns="
        );
        test_encode!(
            b"\xf0y\t\x14\xd161n\x03e\xed\x0e\x05\xdf\xc1\xb9\xda",
            "8HkJFNE2MW4DZe0OBd/Budo="
        );
        test_encode!(
            b"*.\x8e\x1d@\x1ac\xdd;\x9a\xcc \x0c\xc2KI",
            "Ki6OHUAaY907mswgDMJLSQ=="
        );
        test_encode!(b"\xd6\x829\x82\xbc\x00\xc9\xfe\x03", "1oI5grwAyf4D");
        test_encode!(
            b"\r\xf2\xb4\xd4\xa1g\x8fhl\xaa@\x98\x00\xda\x95",
            "DfK01KFnj2hsqkCYANqV"
        );
        test_encode!(
            b"\x1a\xfaV\x1a\xc2e\xc0\xad\xef|\x07\xcf\xa9\xb7O",
            "GvpWGsJlwK3vfAfPqbdP"
        );
        test_encode!(b"\xc20{_\x81\xac", "wjB7X4Gs");
        test_encode!(
            b"B\xa85\xac\xe9\x0ev-\x8bT\xb3|\xde",
            "Qqg1rOkOdi2LVLN83g=="
        );
        test_encode!(
            b"\x05\xe0\xeeSs\xfdY9\x0b7\x84\xfc-\xec",
            "BeDuU3P9WTkLN4T8Lew="
        );
        test_encode!(
            b"Qj\x92\xfa?\xa5\xe3_[\xde\x82\x97{$\xb2\xf9\xd5\x98\x0cy\x15\xe4R\x8d",
            "UWqS+j+l419b3oKXeySy+dWYDHkV5FKN"
        );
        test_encode!(b"\x853\xe0\xc0\x1d\xc1", "hTPgwB3B");
        test_encode!(b"}2\xd0\x13m\x8d\x8f#\x9c\xf5,\xc7", "fTLQE22NjyOc9SzH");
    }

    #[test]
    fn pregenerated_random_bytes_decode() {
        macro_rules! test_decode {
            ($left: expr, $right: expr) => {
                assert_eq!(
                    base64_decode(&String::from($left)).unwrap(),
                    $right.to_vec()
                );
            };
        }
        test_decode!(
            "0zHJh0T+qrP/74wOb0Q=",
            b"\xd31\xc9\x87D\xfe\xaa\xb3\xff\xef\x8c\x0eoD"
        );
        test_decode!(
            "nw44vPXQLbQu1PA/j+cJey7/LzbLVFkhrjmC",
            b"\x9f\x0e8\xbc\xf5\xd0-\xb4.\xd4\xf0?\x8f\xe7\t{.\xff/6\xcbTY!\xae9\x82"
        );
        test_decode!("fzMVGtP5MZtTpDQ9", b"\x7f3\x15\x1a\xd3\xf91\x9bS\xa44=");
        test_decode!(
            "Nzr10Vu/Vi9QGAMA3M2h7Ec=",
            b"7:\xf5\xd1[\xbfV/P\x18\x03\x00\xdc\xcd\xa1\xecG"
        );
        test_decode!(
            "w8kYPXvECJd3TtqBhD+U5p4=",
            b"\xc3\xc9\x18={\xc4\x08\x97wN\xda\x81\x84?\x94\xe6\x9e"
        );
        test_decode!(
            "jEr4ZRMNj3eo5kfOk2Mq50221w==",
            b"\x8cJ\xf8e\x13\r\x8fw\xa8\xe6G\xce\x93c*\xe7M\xb6\xd7"
        );
        test_decode!(
            "3sR+sn2xFEYufqF6fHOQjWSbBIHykns=",
            b"\xde\xc4~\xb2}\xb1\x14F.~\xa1z|s\x90\x8dd\x9b\x04\x81\xf2\x92{"
        );
        test_decode!(
            "8HkJFNE2MW4DZe0OBd/Budo=",
            b"\xf0y\t\x14\xd161n\x03e\xed\x0e\x05\xdf\xc1\xb9\xda"
        );
        test_decode!(
            "Ki6OHUAaY907mswgDMJLSQ==",
            b"*.\x8e\x1d@\x1ac\xdd;\x9a\xcc \x0c\xc2KI"
        );
        test_decode!("1oI5grwAyf4D", b"\xd6\x829\x82\xbc\x00\xc9\xfe\x03");
        test_decode!(
            "DfK01KFnj2hsqkCYANqV",
            b"\r\xf2\xb4\xd4\xa1g\x8fhl\xaa@\x98\x00\xda\x95"
        );
        test_decode!(
            "GvpWGsJlwK3vfAfPqbdP",
            b"\x1a\xfaV\x1a\xc2e\xc0\xad\xef|\x07\xcf\xa9\xb7O"
        );
        test_decode!("wjB7X4Gs", b"\xc20{_\x81\xac");
        test_decode!(
            "Qqg1rOkOdi2LVLN83g==",
            b"B\xa85\xac\xe9\x0ev-\x8bT\xb3|\xde"
        );
        test_decode!(
            "BeDuU3P9WTkLN4T8Lew=",
            b"\x05\xe0\xeeSs\xfdY9\x0b7\x84\xfc-\xec"
        );
        test_decode!(
            "UWqS+j+l419b3oKXeySy+dWYDHkV5FKN",
            b"Qj\x92\xfa?\xa5\xe3_[\xde\x82\x97{$\xb2\xf9\xd5\x98\x0cy\x15\xe4R\x8d"
        );
        test_decode!("hTPgwB3B", b"\x853\xe0\xc0\x1d\xc1");
        test_decode!("fTLQE22NjyOc9SzH", b"}2\xd0\x13m\x8d\x8f#\x9c\xf5,\xc7");
    }

    #[test]
    fn encode_decode() {
        macro_rules! test_e_d {
            ($text: expr) => {
                assert_eq!(
                    base64_decode(&base64_encode(&$text.to_vec())).unwrap(),
                    $text
                );
            };
        }
        test_e_d!(b"green");
        test_e_d!(b"The quick brown fox jumped over the lazy dog.");
        test_e_d!(b"Lorem Ipsum sit dolor amet.");
        test_e_d!(b"0");
        test_e_d!(b"01");
        test_e_d!(b"012");
        test_e_d!(b"0123");
        test_e_d!(b"0123456789");
    }

    #[test]
    fn decode_encode() {
        macro_rules! test_d_e {
            ($data: expr) => {
                assert_eq!(
                    base64_encode(&base64_decode(&String::from($data)).unwrap()),
                    String::from($data)
                );
            };
        }
        test_d_e!("TG9uZyBsaXZlIGVhc3RlciBlZ2dzIDop");
        test_d_e!("SGFwcHkgSGFja3RvYmVyZmVzdCE=");
        test_d_e!("PVRoZSBBbGdvcml0aG1zPQ==");
    }
}
