pub enum StringOrStr<'s> {
    String(String),
    Str(&'s str),
}

impl<'s> StringOrStr<'s> {
    pub fn create_as_string(src: String) -> Self {
        Self::String(src)
    }

    pub fn create_as_str(src: &'s str) -> Self {
        Self::Str(src)
    }

    pub fn as_str(&'s self) -> &'s str {
        match self {
            StringOrStr::String(result) => result,
            StringOrStr::Str(result) => result,
        }
    }
}
