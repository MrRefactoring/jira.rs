pub(crate) fn encode_path_segment(segment: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    if segment.chars().all(|character| character == '.') && !segment.is_empty() {
        return segment.replace('.', "%2E");
    }

    let mut encoded = String::with_capacity(segment.len());

    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b':' | b'@' => {
                encoded.push(char::from(byte));
            }
            _ => {
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ordinary_identifier_is_left_alone() {
        assert_eq!(encode_path_segment("TEST-1"), "TEST-1");
        assert_eq!(encode_path_segment("10042"), "10042");
        assert_eq!(encode_path_segment("my.property_key~2"), "my.property_key~2");
    }

    #[test]
    fn an_account_id_reaches_jira_the_way_jira_wrote_it() {
        assert_eq!(
            encode_path_segment("557058:f58131cb-b67d-43c7-b30d-6b58d40bd077"),
            "557058:f58131cb-b67d-43c7-b30d-6b58d40bd077",
        );
        assert_eq!(encode_path_segment("qm:1a2b3c:4d5e"), "qm:1a2b3c:4d5e");
    }

    #[test]
    fn a_separator_cannot_escape_its_segment() {
        assert_eq!(encode_path_segment("a/b"), "a%2Fb");
        assert_eq!(encode_path_segment("a?b"), "a%3Fb");
        assert_eq!(encode_path_segment("a#b"), "a%23b");
        assert_eq!(encode_path_segment("../../admin"), "..%2F..%2Fadmin");
    }

    #[test]
    fn a_segment_of_nothing_but_dots_cannot_climb() {
        assert_eq!(encode_path_segment(".."), "%2E%2E");
        assert_eq!(encode_path_segment("."), "%2E");
        assert_eq!(encode_path_segment("..."), "%2E%2E%2E");
        assert_eq!(encode_path_segment(".hidden"), ".hidden");
    }

    #[test]
    fn a_space_and_a_plus_are_told_apart() {
        assert_eq!(encode_path_segment("a b"), "a%20b");
        assert_eq!(encode_path_segment("a+b"), "a%2Bb");
    }

    #[test]
    fn a_non_ascii_name_is_encoded_as_its_utf8_bytes() {
        assert_eq!(encode_path_segment("Ольга"), "%D0%9E%D0%BB%D1%8C%D0%B3%D0%B0");
    }

    #[test]
    fn an_already_encoded_value_is_encoded_again_rather_than_trusted() {
        assert_eq!(encode_path_segment("a%2Fb"), "a%252Fb");
    }

    #[test]
    fn an_empty_segment_stays_empty() {
        assert_eq!(encode_path_segment(""), "");
    }
}
