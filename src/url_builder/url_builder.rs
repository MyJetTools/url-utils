use core::str;

use rust_extensions::{
    remote_endpoint::{RemoteEndpoint, Scheme},
    StrOrString,
};

use super::{UrlBuilderInner, UrlBuilderUnixSocket};

pub enum UrlBuilder {
    TcpBased(UrlBuilderInner),
    UnixSocketBased(UrlBuilderUnixSocket),
}

impl UrlBuilder {
    pub fn new(host_port: &str) -> Self {
        let first = host_port.chars().next().unwrap();
        if first == '/' || first == '~' {
            return Self::UnixSocketBased(UrlBuilderUnixSocket::new(host_port));
        }

        let host_index = host_port.find(':');

        if host_index.is_none() {
            return Self::TcpBased(UrlBuilderInner::new(host_port));
        }

        let host_index = host_index.unwrap();

        let scheme_str = &host_port[..host_index];

        let scheme = Scheme::try_parse(scheme_str);

        match scheme {
            Some(scheme) => {
                if scheme.is_unix_socket() {
                    return Self::UnixSocketBased(UrlBuilderUnixSocket::new(host_port));
                }

                return Self::TcpBased(UrlBuilderInner::new(host_port));
            }
            None => Self::TcpBased(UrlBuilderInner::new(host_port)),
        }
    }

    pub fn get_remote_endpoint<'s>(&'s self, default_port: Option<u16>) -> RemoteEndpoint<'s> {
        match self {
            UrlBuilder::TcpBased(url_builder_inner) => {
                url_builder_inner.get_remote_endpoint(default_port)
            }
            UrlBuilder::UnixSocketBased(url_builder_unix_socket) => {
                url_builder_unix_socket.get_remote_endpoint()
            }
        }
    }

    pub fn append_path_segment(&mut self, path: &str) {
        match self {
            UrlBuilder::TcpBased(url_builder_inner) => {
                url_builder_inner.append_path_segment(path);
            }
            UrlBuilder::UnixSocketBased(builder) => {
                builder.append_path_segment(path);
            }
        }
    }

    pub fn append_query_param(&mut self, param: &str, value: Option<&str>) {
        match self {
            UrlBuilder::TcpBased(url_builder_inner) => {
                url_builder_inner.append_query_param(param, value);
            }
            UrlBuilder::UnixSocketBased(builder) => {
                builder.append_query_param(param, value);
            }
        }
    }

    pub fn append_raw_ending(&mut self, raw_ending: &str) {
        match self {
            UrlBuilder::TcpBased(builder) => {
                builder.append_raw_ending(raw_ending);
            }
            UrlBuilder::UnixSocketBased(builder) => {
                builder.append_raw_ending(raw_ending);
            }
        }
    }

    pub fn get_scheme(&self) -> Scheme {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_scheme(),
            UrlBuilder::UnixSocketBased(_) => Scheme::UnixSocket,
        }
    }

    pub fn get_host(&self) -> &str {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_host(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_host(),
        }
    }

    pub fn get_host_port(&self) -> &str {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_host_port(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_host(),
        }
    }

    pub fn get_scheme_and_host(&self) -> &str {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_scheme_and_host(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_host(),
        }
    }

    pub fn get_path_and_query(&self) -> String {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_path_and_query().to_string(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_path_and_query(),
        }
    }
    pub fn host_is_ip(&self) -> bool {
        match self {
            UrlBuilder::TcpBased(builder) => builder.host_is_ip(),
            UrlBuilder::UnixSocketBased(_) => false,
        }
    }

    pub fn get_path(&self) -> &str {
        match self {
            UrlBuilder::TcpBased(builder) => builder.get_path(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_path(),
        }
    }

    pub fn is_unix_socket(&self) -> bool {
        match self {
            UrlBuilder::TcpBased(_) => false,
            UrlBuilder::UnixSocketBased(_) => true,
        }
    }

    pub fn iter_query<'s>(
        &'s self,
    ) -> Option<impl Iterator<Item = (&'s str, Option<StrOrString<'s>>)>> {
        let item = match self {
            UrlBuilder::TcpBased(builder) => builder.get_query(),
            UrlBuilder::UnixSocketBased(builder) => builder.get_query(),
        }?;

        let result = item.split('&').map(|pair| {
            let mut parts = pair.split('=');
            let key = parts.next().unwrap();
            match parts.next() {
                None => (key, None),
                Some(value) => {
                    let value = crate::decode_from_url_string(value);
                    (key, Some(value))
                }
            }
        });

        Some(result)
    }

    pub fn to_string(&self) -> String {
        match self {
            UrlBuilder::TcpBased(builder) => builder.to_string(),
            UrlBuilder::UnixSocketBased(builder) => builder.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::UrlBuilder;

    #[test]
    pub fn test_with_default_scheme() {
        let uri_builder = UrlBuilder::new("google.com".into());

        assert_eq!("http://google.com", uri_builder.to_string());
        assert_eq!("http://google.com", uri_builder.get_scheme_and_host());
        assert_eq!("google.com", uri_builder.get_host());

        assert_eq!(true, uri_builder.get_scheme().is_http());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/", uri_builder.get_path());

        assert_eq!("/", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_with_http_scheme() {
        let uri_builder = UrlBuilder::new("http://google.com".into());

        assert_eq!("http://google.com", uri_builder.to_string());
        assert_eq!("http://google.com", uri_builder.get_scheme_and_host());
        assert_eq!(true, uri_builder.get_scheme().is_http());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/", uri_builder.get_path());
        assert_eq!("/", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_with_http_scheme_and_last_slash() {
        let uri_builder = UrlBuilder::new("http://google.com/".into());

        assert_eq!("http://google.com/", uri_builder.to_string());
        assert_eq!("http://google.com", uri_builder.get_scheme_and_host());
        assert_eq!(true, uri_builder.get_scheme().is_http());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/", uri_builder.get_path());
        assert_eq!("/", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_with_https_scheme() {
        let uri_builder = UrlBuilder::new("https://google.com".into());

        assert_eq!("https://google.com", uri_builder.to_string());
        assert_eq!("https://google.com", uri_builder.get_scheme_and_host());

        assert_eq!(true, uri_builder.get_scheme().is_https());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/", uri_builder.get_path());
        assert_eq!("/", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_path_segments() {
        let mut uri_builder = UrlBuilder::new("https://google.com".into());

        uri_builder.append_path_segment("first");

        uri_builder.append_path_segment("second");

        assert_eq!("https://google.com/first/second", uri_builder.to_string());
        assert_eq!("https://google.com", uri_builder.get_scheme_and_host());

        assert_eq!(true, uri_builder.get_scheme().is_https());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/first/second", uri_builder.get_path());
        assert_eq!("/first/second", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_path_segments_with_slug_at_the_end() {
        let mut uri_builder = UrlBuilder::new("https://google.com/".into());

        uri_builder.append_path_segment("first");
        uri_builder.append_path_segment("second");

        assert_eq!("https://google.com/first/second", uri_builder.to_string());
        assert_eq!("https://google.com", uri_builder.get_scheme_and_host());

        assert_eq!(true, uri_builder.get_scheme().is_https());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/first/second", uri_builder.get_path());
        assert_eq!("/first/second", uri_builder.get_path_and_query());
    }

    #[test]
    pub fn test_query_with_no_path() {
        let mut uri_builder = UrlBuilder::new("https://google.com".into());
        uri_builder.append_query_param("first", Some("first_value"));
        uri_builder.append_query_param("second", Some("second_value"));

        assert_eq!(
            "https://google.com?first=first_value&second=second_value",
            uri_builder.to_string()
        );
        assert_eq!("https://google.com", uri_builder.get_scheme_and_host());

        assert_eq!(true, uri_builder.get_scheme().is_https());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!(uri_builder.get_path(), "/",);
        assert_eq!(
            "?first=first_value&second=second_value",
            uri_builder.get_path_and_query()
        );

        let mut query = uri_builder.iter_query().unwrap();

        let (key, value) = query.next().unwrap();
        assert_eq!("first", key);
        assert_eq!("first_value", value.unwrap().as_str());

        let (key, value) = query.next().unwrap();
        assert_eq!("second", key);
        assert_eq!("second_value", value.unwrap().as_str());

        assert!(query.next().is_none());
    }

    #[test]
    pub fn test_get_domain_different_cases() {
        let uri_builder = UrlBuilder::new("https://my-domain:5123".into());

        assert_eq!("my-domain:5123", uri_builder.get_host_port());
        assert_eq!("my-domain", uri_builder.get_host());

        let uri_builder = UrlBuilder::new("https://my-domain:5123/my-path".into());

        assert_eq!("my-domain:5123", uri_builder.get_host_port());
        assert_eq!("my-domain", uri_builder.get_host());

        let uri_builder = UrlBuilder::new("https://my-domain/my-path".into());

        assert_eq!("my-domain", uri_builder.get_host_port());
        assert_eq!("my-domain", uri_builder.get_host());
    }

    #[test]
    pub fn test_path_and_query() {
        let mut uri_builder = UrlBuilder::new("https://google.com".into());
        uri_builder.append_path_segment("first");
        uri_builder.append_path_segment("second");

        uri_builder.append_query_param("first", Some("first_value"));
        uri_builder.append_query_param("second", Some("second_value"));

        assert_eq!(
            "https://google.com/first/second?first=first_value&second=second_value",
            uri_builder.to_string()
        );
        assert_eq!("https://google.com", uri_builder.get_scheme_and_host());

        assert_eq!(true, uri_builder.get_scheme().is_https());
        assert_eq!("google.com", uri_builder.get_host_port());
        assert_eq!("/first/second", uri_builder.get_path());
        assert_eq!(
            "/first/second?first=first_value&second=second_value",
            uri_builder.get_path_and_query()
        );
    }

    #[test]

    pub fn test_unix_path_and_query() {
        let mut uri_builder = UrlBuilder::new("http+unix://var/run/docker.sock".into());

        uri_builder.append_path_segment("containers");
        uri_builder.append_path_segment("json");

        uri_builder.append_query_param("all", Some("true"));

        assert_eq!(true, uri_builder.get_scheme().is_unix_socket());

        assert_eq!(
            "http+unix://var/run/docker.sock/containers/json?all=true",
            uri_builder.to_string()
        );

        assert_eq!("docker.sock", uri_builder.get_host());

        assert_eq!("docker.sock", uri_builder.get_host_port());

        assert_eq!(
            "/containers/json?all=true",
            uri_builder.get_path_and_query()
        );
    }

    #[test]
    fn test_example_from_real_life() {
        let url = UrlBuilder::new("https://oauth2.googleapis.com/token");

        assert_eq!(url.get_host(), "oauth2.googleapis.com");
        assert!(url.get_scheme().is_https());
        assert_eq!(url.get_host_port(), "oauth2.googleapis.com");

        let remote_host = url.get_remote_endpoint(Some(443));

        assert_eq!(remote_host.get_host(), "oauth2.googleapis.com");
        assert!(remote_host.get_scheme().unwrap().is_https());
        assert_eq!(
            remote_host.get_host_port().as_str(),
            "oauth2.googleapis.com:443"
        );
    }

    #[test]
    fn test_default_http_scheme() {
        let url = UrlBuilder::new("localhost:8080");

        assert_eq!(url.get_host(), "localhost");
        assert!(url.get_scheme().is_http());
        assert_eq!(url.get_host_port(), "localhost:8080");
    }

    #[test]
    fn test_default_http_scheme_and_path() {
        let url = UrlBuilder::new("localhost:8080/templates");

        assert_eq!(url.get_host(), "localhost");
        assert!(url.get_scheme().is_http());
        assert_eq!(url.get_host_port(), "localhost:8080");
        assert_eq!(url.get_path(), "/templates");
    }

    #[test]
    fn test_append_raw_ending() {
        let mut url = UrlBuilder::new("localhost:8080");
        url.append_raw_ending("/path1/path?test=1");

        assert_eq!(url.get_host(), "localhost");

        assert_eq!(url.get_host_port(), "localhost:8080");
        assert_eq!(url.get_path_and_query(), "/path1/path?test=1");

        assert_eq!(url.get_path(), "/path1/path");
    }

    #[test]
    fn test_unix_socket_url() {
        let url = UrlBuilder::new("/unix-socket/directory:/path1/path2");

        assert_eq!(url.get_host_port(), "/unix-socket/directory");
        assert_eq!(url.get_path(), "/path1/path2");
        assert_eq!(url.get_path_and_query(), "/path1/path2");
    }

    #[test]
    fn test_unix_socket_url_2() {
        let url = UrlBuilder::new("/unix-socket/directory:/path1/path2?a=5");

        assert_eq!(url.get_host_port(), "/unix-socket/directory");
        assert_eq!(url.get_path(), "/path1/path2");
        assert_eq!(url.get_path_and_query(), "/path1/path2?a=5");
    }
}
