use rust_extensions::{slice_of_u8_utils::SliceOfU8Ext, MaybeShortString};

use crate::server::ReadingFromDataError;

use super::ContentDispositionParser;

#[derive(Debug)]
pub enum FormDataItem<'s> {
    ValueAsString {
        name: &'s str,
        value: &'s str,
    },
    File {
        name: &'s str,
        file_name: &'s str,
        content_type: &'s str,
        content: &'s [u8],
    },
}

impl<'s> FormDataItem<'s> {
    pub fn unwrap_as_string(&'s self) -> Result<&'s str, ReadingFromDataError> {
        match self {
            FormDataItem::ValueAsString { value, .. } => Ok(value),
            FormDataItem::File { name, .. } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Can not unwrap as string. It is file".to_string(),
            }),
        }
    }

    pub fn unwrap_as_file_name(&'s self) -> &'s str {
        match self {
            FormDataItem::ValueAsString { .. } => {
                panic!("Can not unwrap FormDataItem as string. It is file")
            }
            FormDataItem::File { file_name, .. } => file_name,
        }
    }

    pub fn get_name(&'s self) -> &'s str {
        match self {
            FormDataItem::ValueAsString { name, .. } => name,
            FormDataItem::File { name, .. } => name,
        }
    }
    pub fn parse(src: &'s [u8]) -> Self {
        let mut content_type = None;
        let content;
        let mut file_name = None;
        let mut name = None;

        let mut pos = 0;

        loop {
            if &src[pos..pos + 4] == &[13u8, 10u8, 13u8, 10u8] {
                pos += 4;

                content = Some(&src[pos..src.len() - 2]);
                break;
            }

            pos = src.find_pos_by_condition(pos, |p| p > 32).unwrap();

            let double_quote_pos = src.find_byte_pos(b':' as u8, pos);

            let Some(double_quote_pos) = double_quote_pos else {
                panic!("Invalid form data parsing. Can not find ':'");
            };

            let header_name = MaybeShortString::from_str_as_lower_case(
                std::str::from_utf8(&src[pos..double_quote_pos]).unwrap(),
            );

            let header_name = header_name.as_str();

            match header_name {
                "content-disposition" => {
                    let content_disposition_data = &src[double_quote_pos + 1..];

                    let Some(end) = content_disposition_data.iter().position(|p| *p == 13) else {
                        panic!("Expected byte=13");
                    };

                    let content_disposition_data = &content_disposition_data[..end];

                    for itm in ContentDispositionParser::new(content_disposition_data) {
                        match itm.key {
                            "name" => name = itm.value,
                            "filename" => file_name = itm.value,
                            _ => {}
                        }
                    }

                    pos = src.find_byte_pos(13, double_quote_pos).unwrap();
                }
                "content-type" => {
                    let start = src
                        .find_pos_by_condition(double_quote_pos + 1, |p| p > 32)
                        .unwrap();
                    let end = src.find_pos_by_condition(start, |p| p == 13).unwrap();
                    content_type = Some(std::str::from_utf8(&src[start..end]).unwrap());

                    pos = end;
                }
                _ => {
                    pos = src.find_byte_pos(13, double_quote_pos).unwrap();
                }
            }
        }

        if let Some(content_type) = content_type {
            Self::File {
                file_name: file_name.unwrap(),
                content_type: content_type,
                content: content.unwrap(),
                name: name.unwrap(),
            }
        } else {
            Self::ValueAsString {
                value: {
                    let content = content.unwrap();

                    if content.len() == 0 {
                        ""
                    } else {
                        std::str::from_utf8(content).unwrap()
                    }
                },
                name: name.unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::server::FormDataItem;

    #[test]
    fn test_value_as_string_parser() {
        let msg = vec![
            67, 111, 110, 116, 101, 110, 116, 45, 68, 105, 115, 112, 111, 115, 105, 116, 105, 111,
            110, 58, 32, 102, 111, 114, 109, 45, 100, 97, 116, 97, 59, 32, 110, 97, 109, 101, 61,
            34, 100, 116, 70, 114, 111, 109, 34, 13, 10, 13, 10, 50, 13, 10,
        ];

        let item = super::FormDataItem::parse(&msg);

        match item {
            FormDataItem::ValueAsString { value, name } => {
                assert_eq!(name, "dtFrom");
                assert_eq!(value, "2");
            }
            FormDataItem::File { .. } => {
                panic!("Should be value as string");
            }
        }
    }

    #[test]
    fn test_value_file_parser() {
        let msg = vec![
            67, 111, 110, 116, 101, 110, 116, 45, 68, 105, 115, 112, 111, 115, 105, 116, 105, 111,
            110, 58, 32, 102, 111, 114, 109, 45, 100, 97, 116, 97, 59, 32, 110, 97, 109, 101, 61,
            34, 102, 105, 108, 101, 34, 59, 32, 102, 105, 108, 101, 110, 97, 109, 101, 61, 34, 116,
            101, 115, 116, 45, 112, 97, 121, 108, 111, 97, 100, 46, 116, 120, 116, 34, 13, 10, 67,
            111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 116, 101, 120, 116, 47,
            112, 108, 97, 105, 110, 13, 10, 13, 10, 49, 50, 51, 13, 10,
        ];

        println!("{:?}", std::str::from_utf8(msg.as_slice()));

        let item = super::FormDataItem::parse(&msg);

        match item {
            FormDataItem::ValueAsString { value: _, name: _ } => {
                panic!("Should be value as string");
            }
            FormDataItem::File {
                name,
                file_name,
                content_type,
                content,
            } => {
                assert_eq!(name, "file");
                assert_eq!(file_name, "test-payload.txt");
                assert_eq!(content_type, "text/plain");
                assert_eq!(std::str::from_utf8(content).unwrap(), "123");
            }
        }
    }

    #[test]
    pub fn test_content_disposition_test_real_data() {
        let src: Vec<u8> = vec![
            67, 111, 110, 116, 101, 110, 116, 45, 68, 105, 115, 112, 111, 115, 105, 116, 105, 111,
            110, 58, 32, 102, 111, 114, 109, 45, 100, 97, 116, 97, 59, 32, 110, 97, 109, 101, 61,
            34, 100, 111, 99, 73, 100, 34, 13, 10, 13, 10, 48, 13, 10,
        ];

        println!("src: {:?}", std::str::from_utf8(src.as_slice()).unwrap());

        let result = super::FormDataItem::parse(&src);

        assert_eq!(result.get_name(), "docId");
        assert_eq!(result.unwrap_as_string().unwrap(), "0");
    }

    #[test]
    pub fn test_content() {
        let src = "Content-Disposition: form-data; name=\"file_title\"\n\ntest\n";

        let src = format_text_with_cl_cr(src);
        let result = super::FormDataItem::parse(src.as_slice());

        assert_eq!(result.get_name(), "file_title");
        assert_eq!(result.unwrap_as_string().unwrap(), "test");
    }

    #[test]
    pub fn test_content_with_empty_value() {
        let src = "Content-Disposition: form-data; name=\"file_title\"\n\n\n";

        let src = format_text_with_cl_cr(src);
        let result = super::FormDataItem::parse(src.as_slice());

        assert_eq!(result.get_name(), "file_title");
        assert_eq!(result.unwrap_as_string().unwrap(), "");
    }

    fn format_text_with_cl_cr(src: &str) -> Vec<u8> {
        let mut result = Vec::new();

        for c in src.as_bytes() {
            let c = *c;
            if c == b'\n' {
                result.push(13);
                result.push(10);
            } else {
                result.push(c);
            }
        }

        result
    }
}
