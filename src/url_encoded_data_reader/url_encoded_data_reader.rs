use crate::url_decoder::UrlDecodeError;

use super::{ReadingEncodedDataError, UrlEncodedValue};

pub struct UrlEncodedDataReader<'s> {
    query_string: Vec<UrlEncodedValue<'s>>,
}

impl<'s> UrlEncodedDataReader<'s> {
    pub fn new(src: &'s str) -> Result<Self, UrlDecodeError> {
        let mut query_string = Vec::new();
        let elements = src.split("&");

        for el in elements {
            let kv = el.find('=');

            if let Some(index) = kv {
                let key = crate::url_decoder::decode_from_url_query_string(&el[..index])?;
                let value = UrlEncodedValue::new(key, &el[index + 1..]);
                query_string.push(value);
            }
        }

        let result = Self { query_string };

        Ok(result)
    }

    pub fn get_required(
        &'s self,
        name: &str,
    ) -> Result<UrlEncodedValue<'s>, ReadingEncodedDataError> {
        let result = self.get_optional(name);

        match result {
            Some(e) => Ok(e),
            None => Err(ReadingEncodedDataError::RequiredParameterIsMissing(
                name.to_string(),
            )),
        }
    }

    pub fn get_optional(&'s self, name: &str) -> Option<UrlEncodedValue<'s>> {
        for itm in &self.query_string {
            if itm.get_name() == name {
                return Some(itm.clone());
            }
        }
        None
    }

    pub fn get_vec(&'s self, name: &str) -> Vec<UrlEncodedValue<'s>> {
        let mut result = Vec::new();
        for itm in &self.query_string {
            if itm.get_name() == name {
                result.push(itm.clone());
            }
        }

        result
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

    #[test]
    pub fn test_vec() {
        let query_string =
            "tableName=deposit-restrictions&param[]=1&param[]=2&param[]=3&param[]=4&param[]=5";

        let query_string = UrlEncodedDataReader::new(query_string).unwrap();

        let mut result = Vec::new();

        for itm in query_string.get_vec("param") {
            result.push(itm.as_string().unwrap());
        }

        assert_eq!(vec!["1", "2", "3", "4", "5"], result);
    }

    #[test]
    pub fn test_vec_of_usize() {
        let query_string =
            "tableName=deposit-restrictions&param[]=1&param[]=2&param[]=3&param[]=4&param[]=5&prm[]=1&prm[]=2&prm[]=3&prm[]=4";

        let query_string = UrlEncodedDataReader::new(query_string).unwrap();

        let mut result: Vec<usize> = Vec::new();

        for itm in query_string.get_vec("param") {
            result.push(itm.parse().unwrap());
        }

        assert_eq!(vec![1, 2, 3, 4, 5], result);

        let mut result: Vec<i32> = Vec::new();

        for itm in query_string.get_vec("prm") {
            result.push(itm.parse().unwrap());
        }

        assert_eq!(vec![1, 2, 3, 4], result);

        let mut result: Vec<i32> = Vec::new();

        for itm in query_string.get_vec("params") {
            result.push(itm.parse().unwrap());
        }

        assert_eq!(0, result.len());
    }
}
