use rust_extensions::StrOrString;

use crate::body::{FormDataBody, UrlEncodedBody};

pub enum HttpRequestBody {
    Json(Vec<u8>),
    UrlEncoded(UrlEncodedBody),
    FormData(FormDataBody),
    Raw {
        data: Vec<u8>,
        content_type: Option<&'static str>,
    },
    Empty,
}

impl HttpRequestBody {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn from_raw_data(data: Vec<u8>, content_type: Option<&'static str>) -> Self {
        Self::Raw { data, content_type }
    }

    pub fn as_json(value: &impl serde::Serialize) -> Self {
        let payload = serde_json::to_vec(value).expect("Failed to serialize to JSON");
        Self::Json(payload)
    }

    pub fn get_content_type(&self) -> Option<StrOrString<'static>> {
        match self {
            Self::Json(_) => Some("application/json".into()),
            Self::UrlEncoded(_) => Some("application/x-www-form-urlencoded".into()),
            Self::FormData(body) => Some(body.get_content_type().into()),
            Self::Raw { content_type, .. } => {
                let content_type = (*content_type)?;
                Some(content_type.into())
            }
            Self::Empty => None,
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Self::Json(data) => data,
            Self::UrlEncoded(body) => body.data.into_bytes(),
            Self::FormData(body) => body.into_bytes(),
            Self::Raw { data, .. } => data,
            Self::Empty => Vec::new(),
        }
    }
}

impl Into<HttpRequestBody> for UrlEncodedBody {
    fn into(self) -> HttpRequestBody {
        HttpRequestBody::UrlEncoded(self)
    }
}

impl Into<HttpRequestBody> for FormDataBody {
    fn into(self) -> HttpRequestBody {
        HttpRequestBody::FormData(self)
    }
}
