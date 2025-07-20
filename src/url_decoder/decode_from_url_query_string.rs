use rust_extensions::StrOrString;

use super::{UrlDecodeError, UrlDecoder};

pub fn decode_from_url_query_string<'s>(src: &'s str) -> Result<String, UrlDecodeError> {
    if !has_escape(src.as_bytes()) {
        return Ok(src.to_string());
    }

    let mut result: Vec<u8> = Vec::with_capacity(src.len());
    let mut url_decoder = UrlDecoder::new(src);

    while let Some(next_one) = url_decoder.get_next()? {
        result.push(next_one);
    }

    return Ok(String::from_utf8(result).unwrap());
}

pub fn decode_as_str_or_string<'s>(src: &'s str) -> Result<StrOrString<'s>, UrlDecodeError> {
    if !has_escape(src.as_bytes()) {
        return Ok(StrOrString::create_as_str(src));
    }

    let mut result = String::with_capacity(src.len());
    let mut url_decoder = UrlDecoder::new(src);

    while let Some(next_one) = url_decoder.get_next()? {
        result.push(next_one as char);
    }

    return Ok(StrOrString::create_as_string(result));
}

fn has_escape(src: &[u8]) -> bool {
    for itm in src {
        if *itm == b'%' {
            return true;
        }

        if *itm == b'+' {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_url_decoding() {
        let value = "http%3A%2F%2F127.0.0.1%3A5223";

        let result = super::decode_from_url_query_string(value);

        assert_eq!("http://127.0.0.1:5223", result.unwrap().as_str());
    }

    #[test]
    fn test_url_decoding_case_2() {
        let value = "Euro%20Stoxx%2050";

        let result = super::decode_from_url_query_string(value);

        assert_eq!("Euro Stoxx 50", result.unwrap().as_str());
    }
}
