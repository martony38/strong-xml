use std::borrow::Cow;

fn escape_chars(c: char) -> bool {
    c == '<' || c == '>' || c == '&' || c == '\'' || c == '"'
}

pub fn xml_escape(raw: &str) -> Cow<'_, str> {
    let bytes = raw.as_bytes();

    if let Some(off) = raw.find(escape_chars) {
        let mut result = String::with_capacity(raw.len());

        result.push_str(&raw[0..off]);

        let mut pos = off + 1;

        match bytes[pos - 1] {
            b'<' => result.push_str("&lt;"),
            b'>' => result.push_str("&gt;"),
            b'&' => result.push_str("&amp;"),
            b'\'' => result.push_str("&apos;"),
            b'"' => result.push_str("&quot;"),
            _ => unreachable!(),
        }

        while let Some(off) = raw[pos..].find(escape_chars) {
            result.push_str(&raw[pos..pos + off]);

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => result.push_str("&lt;"),
                b'>' => result.push_str("&gt;"),
                b'&' => result.push_str("&amp;"),
                b'\'' => result.push_str("&apos;"),
                b'"' => result.push_str("&quot;"),
                _ => unreachable!(),
            }
        }

        result.push_str(&raw[pos..]);

        Cow::Owned(result)
    } else {
        Cow::Borrowed(raw)
    }
}

#[test]
fn test_escape() {
    assert_eq!(xml_escape("< < <"), "&lt; &lt; &lt;");
    assert_eq!(
        xml_escape("<script>alert('Hello XSS')</script>"),
        "&lt;script&gt;alert(&apos;Hello XSS&apos;)&lt;/script&gt;"
    );
}
