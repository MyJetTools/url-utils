use std::collections::HashMap;

use crate::url_decoder::UrlDecodeError;

use super::{ReadingEncodedDataError, UrlEncodedValueAsString};

pub struct UrlEncodedDataReader<'s> {
    query_string: HashMap<String, UrlEncodedValueAsString<'s>>,
}

impl<'s> UrlEncodedDataReader<'s> {
    pub fn new(src: &'s str) -> Result<Self, UrlDecodeError> {
        let result = Self {
            query_string: super::url_utils::parse_query_string(src)?,
        };

        Ok(result)
    }

    pub fn get_required(
        &'s self,
        name: &str,
    ) -> Result<&'s UrlEncodedValueAsString<'s>, ReadingEncodedDataError> {
        let result = self.query_string.get(name);

        match result {
            Some(e) => Ok(e),
            None => Err(ReadingEncodedDataError::RequiredParameterIsMissing),
        }
    }

    pub fn get_optional(&'s self, name: &str) -> Option<&'s UrlEncodedValueAsString<'s>> {
        self.query_string.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_basic() {
        let query_string =
            "tableName=deposit-restrictions&partitionKey=%2A&rowKey=1abfc&field=1a+bfc";

        let query_string = UrlEncodedDataReader::new(query_string).unwrap();

        let result = query_string
            .get_optional("partitionKey")
            .unwrap()
            .as_string()
            .unwrap();

        assert_eq!("*", result);

        let result = query_string
            .get_optional("rowKey")
            .unwrap()
            .as_string()
            .unwrap();

        assert_eq!("1abfc", result);

        let result = query_string
            .get_optional("field")
            .unwrap()
            .as_string()
            .unwrap();

        assert_eq!("1a bfc", result);
    }
}
