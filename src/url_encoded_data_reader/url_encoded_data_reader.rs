use std::{collections::HashMap, str::FromStr};

use crate::url_decoder::UrlDecodeError;

use super::{ReadingEncodedDataError, UrlEncodedValueAsString};

pub struct UrlEncodedDataReader<'s> {
    query_string: HashMap<String, UrlEncodedValueAsString<'s>>,
    data_as_vec: Option<HashMap<String, Vec<UrlEncodedValueAsString<'s>>>>,
}

impl<'s> UrlEncodedDataReader<'s> {
    pub fn new(src: &'s str) -> Result<Self, UrlDecodeError> {
        let mut query_string = HashMap::new();
        let mut data_as_vec = None;
        let elements = src.split("&");

        for el in elements {
            let kv = el.find('=');

            if let Some(index) = kv {
                let key = crate::url_decoder::decode_from_url_query_string(&el[..index])?;
                let value = UrlEncodedValueAsString::new(&el[index + 1..]);

                if key.ends_with("[]") {
                    let key = key[..key.len() - 2].to_string();

                    if data_as_vec.is_none() {
                        data_as_vec = Some(HashMap::new());
                    }

                    let data_as_vec = data_as_vec.as_mut().unwrap();

                    if !data_as_vec.contains_key(&key) {
                        data_as_vec.insert(key.clone(), Vec::new());
                    }

                    let data_as_vec = data_as_vec.get_mut(&key).unwrap();

                    data_as_vec.push(value);
                } else {
                    query_string.insert(key, value);
                }
            }
        }

        let result = Self {
            query_string,
            data_as_vec,
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
            None => Err(ReadingEncodedDataError::RequiredParameterIsMissing(
                name.to_string(),
            )),
        }
    }

    pub fn get_optional(&'s self, name: &str) -> Option<&'s UrlEncodedValueAsString<'s>> {
        self.query_string.get(name)
    }

    pub fn get_vec_of_string(&'s self, name: &str) -> Result<Vec<String>, ReadingEncodedDataError> {
        if self.data_as_vec.is_none() {
            return Ok(vec![]);
        }

        if let Some(values) = self.data_as_vec.as_ref().unwrap().get(name) {
            let mut result = Vec::new();

            for itm in values {
                result.push(itm.as_string()?);
            }

            return Ok(result);
        }

        Ok(vec![])
    }

    pub fn get_vec<TResult: FromStr>(
        &'s self,
        name: &str,
    ) -> Result<Vec<TResult>, ReadingEncodedDataError> {
        if self.data_as_vec.is_none() {
            return Ok(vec![]);
        }

        if let Some(values) = self.data_as_vec.as_ref().unwrap().get(name) {
            let mut result = Vec::new();

            for itm in values {
                result.push(itm.parse()?);
            }

            return Ok(result);
        }

        Ok(vec![])
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

        let result = query_string.get_vec_of_string("param").unwrap();

        assert_eq!(vec!["1", "2", "3", "4", "5"], result);
    }

    #[test]
    pub fn test_vec_of_usize() {
        let query_string =
            "tableName=deposit-restrictions&param[]=1&param[]=2&param[]=3&param[]=4&param[]=5&prm[]=1&prm[]=2&prm[]=3&prm[]=4";

        let query_string = UrlEncodedDataReader::new(query_string).unwrap();

        let result: Vec<usize> = query_string.get_vec("param").unwrap();

        assert_eq!(vec![1, 2, 3, 4, 5], result);

        let result: Vec<i32> = query_string.get_vec("prm").unwrap();

        assert_eq!(vec![1, 2, 3, 4], result);

        let result: Vec<i32> = query_string.get_vec("prms").unwrap();

        assert_eq!(0, result.len());
    }
}
