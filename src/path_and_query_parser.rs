pub struct PathAndQueryReader<'s> {
    pub path: &'s str,
    pub query: Option<&'s str>,
}

impl<'s> PathAndQueryReader<'s> {
    pub fn new(path_and_query: &'s str) -> Self {
        if path_and_query.is_empty() {
            return Self {
                path: "/",
                query: None,
            };
        }

        let mut split = path_and_query.split('?');
        let left = split.next().unwrap();
        let right = split.next();

        Self {
            path: if left.ends_with('/') {
                &left[..left.len() - 1]
            } else {
                left
            },
            query: right,
        }
    }

    pub fn is_my_path(&self, path: &str) -> bool {
        let path = if path.is_empty() { "/" } else { path };

        if path.starts_with("/") {
            return self.path.eq_ignore_ascii_case(path);
        }

        let self_path = &self.path[1..];
        return self_path.eq_ignore_ascii_case(path);
    }
}

#[cfg(test)]
mod tests {
    use crate::PathAndQueryReader;

    #[test]
    fn test_my_path() {
        let src = PathAndQueryReader::new("/my-path?param=1");

        assert!(src.is_my_path("/my-path"));
        assert!(src.is_my_path("/my-Path"));
        assert!(src.is_my_path("my-Path"));
        assert!(!src.is_my_path("my-Path-2"));
    }
}
