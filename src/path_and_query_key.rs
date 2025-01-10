use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathAndQueryKey(String);

impl PathAndQueryKey {
    pub fn from_path_and_query(path_and_query: &str) -> Self {
        let mut iterator = path_and_query.split('?');

        let path = iterator.next().unwrap_or("/");

        let query = iterator.next();

        if query.is_none() {
            return Self(path.to_lowercase());
        }

        let query = query.unwrap();

        let mut query_builder = BTreeMap::new();

        for key_value in query.split("&") {
            let mut key_value_iterator = key_value.split('=');

            let key = key_value_iterator.next().unwrap_or("").to_string();

            let value = key_value_iterator.next();

            query_builder.insert(key, value);
        }

        let mut query_result = String::new();

        for (key, value) in query_builder {
            if query_result.len() > 0 {
                query_result.push('&');
            }
            query_result.push_str(&key);

            if let Some(value) = value {
                query_result.push('=');
                query_result.push_str(&value);
            }
        }

        Self(format!("{}?{}", path.to_lowercase(), query_result))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn has_query(&self) -> bool {
        self.0.contains('?')
    }
}

#[cfg(test)]
mod test {

    use crate::PathAndQueryKey;

    #[test]
    fn test_basic() {
        let path1 = PathAndQueryKey::from_path_and_query("/path/to/some/where?name=John&age=20");

        let path2 = PathAndQueryKey::from_path_and_query("/path/to/Some/where?age=20&name=John");

        assert_eq!(path1.as_str(), path2.as_str());

        assert_eq!(path1.has_query(), true);
    }

    #[test]
    fn test_basic_2() {
        let path1 = PathAndQueryKey::from_path_and_query("/path/to/some/where");

        let path2 = PathAndQueryKey::from_path_and_query("/path/to/Some/where");

        assert_eq!(path1.as_str(), path2.as_str());

        assert_eq!(path1.has_query(), false);
    }
}
