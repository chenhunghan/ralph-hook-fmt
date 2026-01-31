use std::path::PathBuf;

/// Extract file path from hook input JSON
/// Looks for: {"tool_input": {"file_path": "..."}}
pub fn extract_file_path(input: &str) -> Option<PathBuf> {
    // Find "file_path" key and extract its value
    let file_path_key = "\"file_path\"";
    let start = input.find(file_path_key)?;
    let after_key = &input[start + file_path_key.len()..];

    // Skip whitespace and colon
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let after_colon = after_colon.trim_start();

    // Extract string value
    let value_start = after_colon.strip_prefix('"')?;
    let end = value_start.find('"')?;
    let path_str = &value_start[..end];

    // Handle escaped characters
    let path_str = path_str.replace("\\\"", "\"").replace("\\\\", "\\");

    if path_str.is_empty() {
        return None;
    }

    Some(PathBuf::from(path_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_path_write() {
        let input = r#"{
            "tool_name": "Write",
            "tool_input": {
                "file_path": "/path/to/file.rs",
                "content": "fn main() {}"
            }
        }"#;
        let path = extract_file_path(input).unwrap();
        assert_eq!(path, PathBuf::from("/path/to/file.rs"));
    }

    #[test]
    fn test_extract_file_path_edit() {
        let input = r#"{
            "tool_name": "Edit",
            "tool_input": {
                "file_path": "/path/to/file.py",
                "old_string": "foo",
                "new_string": "bar"
            }
        }"#;
        let path = extract_file_path(input).unwrap();
        assert_eq!(path, PathBuf::from("/path/to/file.py"));
    }

    #[test]
    fn test_extract_file_path_missing() {
        let input = r#"{
            "tool_name": "Write",
            "tool_input": {}
        }"#;
        assert!(extract_file_path(input).is_none());
    }

    #[test]
    fn test_extract_file_path_invalid_json() {
        let input = "not valid json";
        assert!(extract_file_path(input).is_none());
    }

    #[test]
    fn test_extract_file_path_with_spaces() {
        let input = r#"{"tool_input": {"file_path": "/path/with spaces/file.rs"}}"#;
        let path = extract_file_path(input).unwrap();
        assert_eq!(path, PathBuf::from("/path/with spaces/file.rs"));
    }

    #[test]
    fn test_extract_file_path_compact_json() {
        let input = r#"{"tool_input":{"file_path":"/path/to/file.rs"}}"#;
        let path = extract_file_path(input).unwrap();
        assert_eq!(path, PathBuf::from("/path/to/file.rs"));
    }
}
