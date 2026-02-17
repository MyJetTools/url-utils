use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::de::DeserializeOwned;

use crate::server::FormDataItem;

use super::ReadingFromDataError;

impl<'s> TryInto<String> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, .. } => {
                return Ok(value.to_string());
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to string".into(),
            }),
        }
    }
}

impl<'s> TryInto<&'s str> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<&'s str, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, .. } => {
                return Ok(*value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field {} contains a File which is not possible to convert to str".into(),
            }),
        }
    }
}

impl<'s> TryInto<bool> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_bool(name, value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to bool".into(),
            }),
        }
    }
}

impl<'s> TryInto<u8> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to u8".into(),
            }),
        }
    }
}

impl<'s> TryInto<i8> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<i8, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to i8".into(),
            }),
        }
    }
}

impl<'s> TryInto<u16> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to u16".into(),
            }),
        }
    }
}

impl<'s> TryInto<i16> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<i16, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to i16".into(),
            }),
        }
    }
}

impl<'s> TryInto<u32> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to u32".into(),
            }),
        }
    }
}

impl<'s> TryInto<i32> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to i32".into(),
            }),
        }
    }
}

impl<'s> TryInto<u64> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to u64".into(),
            }),
        }
    }
}

impl<'s> TryInto<i64> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to i64".into(),
            }),
        }
    }
}

impl<'s> TryInto<f32> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to f32".into(),
            }),
        }
    }
}

impl<'s> TryInto<f64> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to f64".into(),
            }),
        }
    }
}

impl<'s> TryInto<usize> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to usize".into(),
            }),
        }
    }
}

impl<'s> TryInto<isize> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<isize, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_simple_value(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to isize".into(),
            }),
        }
    }
}

impl<'s> TryInto<DateTimeAsMicroseconds> for &'s FormDataItem<'s> {
    type Error = ReadingFromDataError;
    fn try_into(self) -> Result<DateTimeAsMicroseconds, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_date_time(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content: _,
            } => Err(ReadingFromDataError::ValidationError {
                field: name.to_string(),
                error: "Field contains a File which is not possible to convert to usize".into(),
            }),
        }
    }
}

impl<'s, TValue> TryInto<HashMap<String, TValue>> for &'s FormDataItem<'s>
where
    TValue: DeserializeOwned,
{
    type Error = ReadingFromDataError;

    fn try_into(self) -> Result<HashMap<String, TValue>, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_json(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content,
            } => {
                return to_json_from_slice(name, content);
            }
        }
    }
}

impl<'s, TValue> TryInto<Vec<TValue>> for &'s FormDataItem<'s>
where
    TValue: DeserializeOwned,
{
    type Error = ReadingFromDataError;

    fn try_into(self) -> Result<Vec<TValue>, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => {
                return to_json(name, *value);
            }
            FormDataItem::File {
                name,
                file_name: _,
                content_type: _,
                content,
            } => {
                return to_json_from_slice(name, content);
            }
        }
    }
}
/*
impl<'s, T: DeserializeOwned> TryInto<RawDataTyped<T>> for FormDataItem<'s> {
    type Error = HttpFailResult;

    fn try_into(self) -> Result<RawDataTyped<T>, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => match value {
                Some(value) => Ok(RawDataTyped::new(value.as_bytes().to_vec(), SRC_FORM_DATA)),
                None => Err(HttpFailResult::required_parameter_is_missing(
                    name,
                    SRC_FORM_DATA,
                )),
            },

            FormDataItem::File {
                name: _,
                file_name: _,
                content_type: _,
                content,
            } => Ok(RawDataTyped::new(content.to_vec(), SRC_FORM_DATA)),
        }
    }
}

impl<'s> TryInto<RawData> for &'s FormDataItem<'s> {
    type Error = HttpFailResult;

    fn try_into(self) -> Result<RawData, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value, name } => match value {
                Some(value) => Ok(RawData::new(value.as_bytes().to_vec())),
                None => Err(HttpFailResult::required_parameter_is_missing(
                    name,
                    SRC_FORM_DATA,
                )),
            },
            FormDataItem::File {
                name: _,
                file_name: _,
                content_type: _,
                content,
            } => Ok(RawData::new(content.to_vec())),
        }
    }
}


impl<'s> TryInto<FileContent> for &'s FormDataItem<'s> {
    type Error = HttpFailResult;

    fn try_into(self) -> Result<FileContent, Self::Error> {
        match self {
            FormDataItem::ValueAsString { value: _, name } => {
                Err(HttpFailResult::as_not_supported_content_type(format!(
                    "Field {} contains a value which is not possible to convert to a file",
                    name,
                )))
            }
            FormDataItem::File {
                name: _,
                file_name,
                content_type,
                content,
            } => Ok(FileContent {
                content_type: content_type.to_string(),
                file_name: file_name.to_string(),
                content: content.to_vec(),
            }),
        }
    }
}
 */

fn to_bool(param_name: &str, value: &str) -> Result<bool, ReadingFromDataError> {
    if value == "1" || value.to_lowercase() == "true" {
        return Ok(true);
    }

    if value == "0" || value.to_lowercase() == "false" {
        return Ok(false);
    }

    let err = ReadingFromDataError::ValidationError {
        field: param_name.to_string(),
        error: "Can not convert value to boolean".into(),
    };

    return Err(err);
}

fn to_simple_value<T: std::str::FromStr>(
    param_name: &str,
    value: &str,
) -> Result<T, ReadingFromDataError> {
    if let Ok(result) = value.parse() {
        return Ok(result);
    }
    let err = ReadingFromDataError::ValidationError {
        field: param_name.to_string(),
        error: "Can not convert value to simple value".into(),
    };

    return Err(err);
}

fn to_date_time(
    param_name: &str,
    value: &str,
) -> Result<DateTimeAsMicroseconds, ReadingFromDataError> {
    if let Some(result) = DateTimeAsMicroseconds::from_str(value) {
        return Ok(result);
    }

    let err = ReadingFromDataError::ValidationError {
        field: param_name.to_string(),
        error: "Can not convert value to DateTime".into(),
    };

    return Err(err);
}

fn to_json<TResult: DeserializeOwned>(
    param_name: &str,
    value: &str,
) -> Result<TResult, ReadingFromDataError> {
    let result: Result<TResult, _> = serde_json::from_str(value);

    match result {
        Ok(result) => return Ok(result),
        Err(err) => {
            return Err(ReadingFromDataError::ValidationError {
                field: param_name.to_string(),
                error: format!("Can not deserialize from json. Err: {:?}", err),
            })
        }
    }
}

fn to_json_from_slice<TResult: DeserializeOwned>(
    param_name: &str,
    value: &[u8],
) -> Result<TResult, ReadingFromDataError> {
    let result: Result<TResult, _> = serde_json::from_slice(value);

    match result {
        Ok(result) => return Ok(result),
        Err(err) => {
            return Err(ReadingFromDataError::ValidationError {
                field: param_name.to_string(),
                error: format!("Can not deserialize from json. Err: {:?}", err),
            })
        }
    }
}
