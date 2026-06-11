pub fn empty_body() -> serde_json::Value {
    serde_json::json!({})
}

pub fn single_field_body(field: &str, value: impl Into<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({ field: value.into() })
}

pub fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

pub fn path_segment_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes())
        .map(|part| if part == "+" { "%20" } else { part })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_segment_encoding_escapes_path_separators() {
        assert_eq!(path_segment_encode("a b/c?d#e"), "a%20b%2Fc%3Fd%23e");
    }

    #[test]
    fn query_encoding_keeps_existing_plus_space_behavior() {
        assert_eq!(url_encode("a b"), "a+b");
    }
}
