use crate::server::{FormDataItem, ReadingFromDataError};

use super::content_iterator::ContentIterator;

pub struct FormDataReader<'s> {
    data: Vec<FormDataItem<'s>>,
}

impl<'s> FormDataReader<'s> {
    pub fn new(content: &'s [u8], boundary: &'s str) -> Self {
        let mut data = Vec::new();

        for chunk in ContentIterator::new(content, boundary) {
            let item = FormDataItem::parse(chunk);
            data.push(item);
        }

        Self { data }
    }
    pub fn get_required(
        &'s self,
        name: &str,
    ) -> Result<&'s FormDataItem<'s>, ReadingFromDataError> {
        for itm in &self.data {
            if itm.get_name() == name {
                return Ok(itm);
            }
        }

        Err(ReadingFromDataError::ParameterMissing(name.to_string()))
    }

    pub fn get_optional(&'s self, name: &str) -> Option<&'s FormDataItem<'s>> {
        for itm in &self.data {
            if itm.get_name() == name {
                return Some(itm);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::FormDataReader;

    #[test]
    fn test() {
        let payload = std::include_bytes!("../../../test_form_data_payload.txt");

        let payload = format_text_with_cl_cr(payload);

        let form_data_reader = FormDataReader::new(
            payload.as_slice(),
            "------DataFormBoundary15c4050a1c8749f2a53abd41b27c9d0f",
        );

        for itm in form_data_reader.data.iter() {
            println!("{:?}", itm);
        }
    }

    fn format_text_with_cl_cr(src: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();

        for c in src {
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
