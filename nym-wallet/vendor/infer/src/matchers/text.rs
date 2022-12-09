/// Returns whether a buffer is html data.
///
/// Conforms to [whatwg](https://mimesniff.spec.whatwg.org/)
/// specification.
pub fn is_html(buf: &[u8]) -> bool {
    let values: &[&[u8]] = &[
        b"<!DOCTYPE HTML",
        b"<HTML",
        b"<HEAD",
        b"<SCRIPT",
        b"<IFRAME",
        b"<H1",
        b"<DIV",
        b"<FONT",
        b"<TABLE",
        b"<A",
        b"<STYLE",
        b"<TITLE",
        b"<B",
        b"<BODY",
        b"<BR",
        b"<P",
        b"<!--",
    ];
    let buf = trim_start_whitespaces(buf);

    for val in values {
        if starts_with_ignore_ascii_case(buf, val) && buf.len() > val.len() {
            match buf[val.len()] {
                // tag-terminitating byte
                0x20 | 0x3E => return true,
                _ => continue,
            }
        }
    }

    false
}

/// Returns whether a buffer is xml data.
///
/// Conforms to [whatwg](https://mimesniff.spec.whatwg.org/)
/// specification.
pub fn is_xml(buf: &[u8]) -> bool {
    let val: &[u8] = b"<?xml";
    let buf = trim_start_whitespaces(buf);
    starts_with_ignore_ascii_case(buf, val)
}

/// Strip whitespaces at the beginning of the buffer.
///
/// Follows https://mimesniff.spec.whatwg.org
/// definition of whitespace.
fn trim_start_whitespaces(mut buf: &[u8]) -> &[u8] {
    while !buf.is_empty() {
        match buf[0] {
            0x09 | 0x0A | 0x0C | 0x0D | 0x20 => buf = &buf[1..],
            _ => break,
        }
    }
    buf
}

fn starts_with_ignore_ascii_case(buf: &[u8], needle: &[u8]) -> bool {
    buf.len() >= needle.len() && buf[..needle.len()].eq_ignore_ascii_case(needle)
}

/// Returns whether a buffer is a shell script.
pub fn is_shellscript(buf: &[u8]) -> bool {
    buf.len() > 2 && &buf[..2] == b"#!"
}

#[cfg(test)]
mod tests {
    use super::{is_html, is_shellscript, trim_start_whitespaces};

    #[test]
    fn trim_whitespaces() {
        let got = trim_start_whitespaces(&[0x09, 0x0A, 0x0C, 0x0D, 0x20, b'A', b'B', b'C']);
        assert_eq!(got, b"ABC");

        let got = trim_start_whitespaces(b"abc");
        assert_eq!(got, b"abc");

        let got = trim_start_whitespaces(&[]);
        assert_eq!(got, &[]);
    }

    #[test]
    fn html() {
        assert_eq!(is_html(b"<"), false);
        assert_eq!(is_html(b"<HTML"), false);
        assert_eq!(is_html(b"<HTML "), true);
        assert_eq!(is_html(b"   <BODY>"), true);
    }

    #[test]
    fn shellscript() {
        assert_eq!(is_shellscript(b"#!"), false);
    }
}
