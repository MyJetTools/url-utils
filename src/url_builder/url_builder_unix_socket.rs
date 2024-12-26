use rust_extensions::{remote_endpoint::RemoteEndpoint, ShortString};

pub struct UrlBuilderUnixSocket {
    host: ShortString,
    path: String,
    query: String,
    host_index: usize,
}

impl UrlBuilderUnixSocket {
    pub fn new(host_port: &str, host_index: usize) -> Self {
        Self {
            host: ShortString::from_str(host_port).unwrap(),
            path: String::new(),
            query: String::new(),
            host_index,
        }
    }

    pub fn get_remote_endpoint(&self) -> RemoteEndpoint {
        RemoteEndpoint::try_parse(&self.host).unwrap()
    }

    pub fn append_path_segment(&mut self, path_segment: &str) {
        self.path.push('/');
        self.path.push_str(path_segment);
    }

    pub fn append_query_param(&mut self, name: &str, value: Option<&str>) {
        if self.query.is_empty() {
            self.query.push('?');
        } else {
            self.query.push('&');
        }

        crate::encode_to_url_string_and_copy(&mut self.query, name);

        if let Some(value) = value {
            self.query.push('=');
            crate::encode_to_url_string_and_copy(&mut self.query, value);
        }
    }

    pub fn get_path_and_query(&self) -> String {
        let mut path_and_query = self.path.clone();
        path_and_query.push_str(&self.query);
        path_and_query
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_scheme_and_host(&self) -> &str {
        &self.host
    }

    pub fn get_host(&self) -> &str {
        &self.host[self.host_index + 2..]
    }

    pub fn append_raw_ending(&mut self, raw_ending: &str) {
        self.path.push_str(raw_ending);
    }

    pub fn get_query(&self) -> Option<&str> {
        if self.query.is_empty() {
            None
        } else {
            Some(&self.query[1..])
        }
    }

    pub fn to_string(&self) -> String {
        let mut url = String::with_capacity(self.host.len() + self.path.len() + self.query.len());
        url.push_str(&self.host);
        if self.path.len() > 0 {
            url.push_str(&self.path);
        }

        if self.query.len() > 0 {
            url.push_str(&self.query);
        }

        url
    }
}
