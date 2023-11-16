use crate::{url_decoder::UrlDecodeError, url_encoded_data_reader::UrlEncodedValueAsString};

pub fn parse_query_string<'s>(
    query_string: &'s str,
) -> Result<Vec<UrlEncodedValueAsString<'s>>, UrlDecodeError> {
    let mut result = Vec::new();
    let elements = query_string.split("&");

    for el in elements {
        let kv = el.find('=');

        if let Some(index) = kv {
            let key = crate::url_decoder::decode_from_url_query_string(&el[..index])?;
            let value = UrlEncodedValueAsString::new(key, &el[index + 1..]);
            result.push(value);
        }
    }

    Ok(result)
}
