use std::str::FromStr;

use rust_extensions::StrOrString;

use super::ReadingEncodedDataError;

#[derive(Clone)]
pub struct UrlEncodedValue<'s> {
    name: String,
    pub value: &'s str,
}

impl<'s> UrlEncodedValue<'s> {
    pub fn new(name: String, value: &'s str) -> Self {
        Self { name, value }
    }

    pub fn get_name(&self) -> &str {
        if self.name.ends_with("[]") {
            return &self.name[..self.name.len() - 2];
        }

        &self.name
    }

    pub fn as_string(&self) -> Result<String, ReadingEncodedDataError> {
        let result = crate::url_decoder::decode_from_url_query_string(self.value)?;
        Ok(result)
    }

    pub fn as_str_or_string(&'s self) -> Result<StrOrString<'s>, ReadingEncodedDataError> {
        let result = crate::url_decoder::decode_as_str_or_string(self.value)?;
        Ok(result)
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
