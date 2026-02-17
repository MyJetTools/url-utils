# url-utils

Lightweight URL helper library for Rust: percent-encoding/decoding, query-string parsing, and ergonomic accessors for decoded parameters.

## Features
- Parse URL-encoded query strings into typed accessors (`UrlEncodedDataReader`).
- Streaming percent-decoder (`UrlDecoder`) that handles `+` → space and `%XX` escapes.
- Percent-encoder (`encode_string`) that avoids allocations when possible via `StrOrString`.
- Array-style parameter support (`param[]=1&param[]=2` → name `param`).

## Installation
Add the crate as a git dependency (adjust the URL/tag to your fork if needed):
```toml
[dependencies]
url-utils = {tag = "{last_tag}", git = "https://github.com/MyJetTools/url-utils.git" }
```

## Usage

### Read and decode query parameters
```rust
use url_utils::url_encoded_data_reader::UrlEncodedDataReader;

let qs = "tableName=deposit-restrictions&partitionKey=%2A&rowKey=1abfc&param[]=1&param[]=2";
let reader = UrlEncodedDataReader::new(qs)?;

let table = reader.get_required("tableName")?.as_string()?; // "deposit-restrictions"
let partition = reader.get_optional("partitionKey").unwrap().as_string()?; // "*"

let ids: Vec<usize> = reader
    .get_vec("param")
    .into_iter()
    .map(|v| v.parse().unwrap())
    .collect(); // [1, 2]
```

### Parse into raw values
```rust
use url_utils::{parse_query_string, url_encoded_data_reader::UrlEncodedValue};

let items: Vec<UrlEncodedValue> = parse_query_string("k1=v1&k2=hello+world")?;
let v2 = items[1].as_string()?; // "hello world"
```

### Encode strings for use in URLs
```rust
use url_utils::url_encoder::encode_string;

let encoded = encode_string("hello world+?"); // "hello%20world%2B%3F"
```

### Stream-decode percent-encoded data
```rust
use url_utils::url_decoder::UrlDecoder;

let mut decoder = UrlDecoder::new("a+b%20c");
let mut out = Vec::new();
while let Some(b) = decoder.get_next()? {
    out.push(b);
}
assert_eq!(String::from_utf8(out).unwrap(), "a b c");
```

## Tests
Run the existing suite:
```sh
cargo test
```
