use rust_extensions::StrOrString;

pub fn encode_string(src: &str) -> StrOrString {
    let as_bytes = src.as_bytes();
    if !has_to_be_encoded(as_bytes) {
        return StrOrString::create_as_str(src);
    }

    let mut result = String::new();

    for i in 0..as_bytes.len() {
        let b = as_bytes[i] as char;
        if let Some(str) = super::encode_map::URL_ENCODE_SYMBOLS.get(&b) {
            result.push_str(str);
        } else {
            result.push(b)
        }
    }

    StrOrString::create_as_string(result)
}

fn has_to_be_encoded(src: &[u8]) -> bool {
    for i in 0..src.len() {
        let b = src[i] as char;
        if super::encode_map::URL_ENCODE_SYMBOLS.contains_key(&b) {
            return true;
        }
    }

    false
}
