use std::str::FromStr;

use super::ReadingEncodedDataError;

pub struct UrlEncodedValueAsString<'s> {
    pub value: &'s str,
}

impl<'s> UrlEncodedValueAsString<'s> {
    pub fn new(src: &'s str) -> Self {
        Self { value: src }
    }

    pub fn as_string(&self) -> Result<String, ReadingEncodedDataError> {
        let result = crate::url_decoder::decode_from_url_query_string(self.value)?;
        Ok(result)
    }

    pub fn as_bool(&'s self) -> Result<bool, ReadingEncodedDataError> {
        let bool_value = parse_bool_value(self.value)?;
        Ok(bool_value)
    }

    pub fn parse<T: FromStr>(&'s self) -> Result<T, ReadingEncodedDataError> {
        let result = self.value.parse::<T>();
        return match result {
            Ok(value) => Ok(value),
            _ => Err(ReadingEncodedDataError::CanNotParseValue(
                self.value.to_string(),
            )),
        };
    }
}

fn parse_bool_value(value: &str) -> Result<bool, ReadingEncodedDataError> {
    let value = value.to_lowercase();
    if value == "1" || value.to_lowercase() == "true" {
        return Ok(true);
    }

    if value == "0" || value.to_lowercase() == "false" {
        return Ok(false);
    }

    let err = ReadingEncodedDataError::CanNotParseValue(value.to_string());

    return Err(err);
}
