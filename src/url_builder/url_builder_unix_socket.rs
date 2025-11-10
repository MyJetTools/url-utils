use rust_extensions::remote_endpoint::RemoteEndpoint;

pub struct UrlBuilderUnixSocket {
    host: String,
    path: String,
    query: String,
}

impl UrlBuilderUnixSocket {
    pub fn new(host_port: &str) -> Self {
        let index = host_port.find(':');

        let Some(index) = index else {
            return Self {
                host: host_port.to_string(),
                path: Default::default(),
                query: Default::default(),
            };
        };

        let host = host_port[..index].to_string();

        let path_and_query = host_port[index + 1..].to_string();

        let (path, query) = match path_and_query.find('?') {
            Some(index) => {
                let path = path_and_query[..index].to_string();
                let query = path_and_query[index + 1..].to_string();
                (path, query)
            }
            None => (path_and_query.to_string(), String::new()),
        };

        Self { host, path, query }
    }

    pub fn get_remote_endpoint<'s>(&'s self) -> RemoteEndpoint<'s> {
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
        let mut result = String::with_capacity(self.path.len() + self.query.len() + 1);
        if self.path.len() > 0 {
            result.push_str(&self.path);
        }

        if self.query.len() > 0 {
            result.push('?');
            result.push_str(&self.query);
        }
        result
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_scheme_and_host(&self) -> &str {
        &self.host
    }

    pub fn get_host(&self) -> &str {
        self.host.as_str()
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
