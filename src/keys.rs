//! plank's key payload, `{"code": "ctrl-s", "text": "S"}`: the part that is
//! plank's own wire format. `translate` and `is_ctrl`, which turn `code`
//! into a Turbo Vision key event, moved to [`tv_extensions::keys`].

/// The `text` field of a flat key payload, unescaped, when it is one char.
#[must_use]
pub fn payload_text(payload: &str) -> Option<char> {
    let start = payload.find("\"text\":")? + "\"text\":".len();
    let rest = payload[start..].trim_start().strip_prefix('"')?;
    let mut chars = rest.chars();
    let c = match chars.next()? {
        '\\' => match chars.next()? {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            'b' => '\u{8}',
            'f' => '\u{c}',
            'u' => {
                let hex: String = chars.by_ref().take(4).collect();
                char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?
            }
            other => other,
        },
        c => c,
    };
    (chars.next() == Some('"')).then_some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_text_reads_one_char_and_unescapes() {
        assert_eq!(payload_text(r#"{"code": "a", "text": "A"}"#), Some('A'));
        assert_eq!(payload_text(r#"{"code": "\"", "text": "\""}"#), Some('"'));
        assert_eq!(payload_text(r#"{"code": "enter"}"#), None);
    }

    #[test]
    fn payload_text_decodes_control_escapes() {
        assert_eq!(payload_text(r#"{"code": "tab", "text": "\t"}"#), Some('\t'));
        assert_eq!(
            payload_text(r#"{"code": "enter", "text": "\r"}"#),
            Some('\r')
        );
        assert_eq!(
            payload_text(r#"{"code": "a", "text": "\u0001"}"#),
            Some('\u{1}')
        );
    }
}
