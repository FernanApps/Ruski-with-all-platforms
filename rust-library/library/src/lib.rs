use base64::{
    Engine as _,
    engine::general_purpose::STANDARD as base64Engine
};

/// Encrypts a String.
///
/// Example:
/// ```
/// use library::encrypt;
/// assert_eq!(encrypt("hello"), "aGVsbG8=");
/// assert_eq!(encrypt(""), "");
/// ```
pub fn encrypt(to: &str) -> String {
    base64Engine.encode(String::from(to))
}

/// Decrypts a String.
///
/// Example:
/// ```
/// use library::decrypt;
/// assert_eq!(decrypt("aGVsbG8="), "hello");
/// assert_eq!(decrypt(""), "");
/// ```
pub fn decrypt(from: &str) -> String {
    let base64_bytes = base64Engine.decode(
        String::from(from)
    ).unwrap_or(vec![]);

    match String::from_utf8(base64_bytes) {
        Ok(result) => result,
        Err(_) => "".to_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        let to_encrypt = "hello_world_from_rust";
        let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";

        let encrypted_result = encrypt(&to_encrypt);

        assert_eq!(str_encoded_b64, encrypted_result);
    }

    #[test]
    fn test_decrypt_string() {
        let str_encoded_b64 = "aGVsbG9fd29ybGRfZnJvbV9ydXN0";
        let str_decoded_b64 = "hello_world_from_rust";

        let decrypted_result = decrypt(&str_encoded_b64);

        assert_eq!(str_decoded_b64, decrypted_result);
    }


    #[test]
    fn test_decrypt_empty_string() {
        let empty_str = "";

        let decrypted_result: String = decrypt(&empty_str);

        assert_eq!(empty_str, decrypted_result)
    }

    #[test]
    fn test_decrypt_invalid_base64_string() {
        let invalid_base64_str = "dfoiuerw892";

        let decrypted_result: String = decrypt(&invalid_base64_str);

        assert_eq!("", decrypted_result)
    }
}
