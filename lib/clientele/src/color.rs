// This is free and unencumbered software released into the public domain.

/// Strips the ANSI CSI escape sequences emitted by `color_print`.
///
/// Available without optional features. Returns an owned string with complete
/// CSI sequences removed, preserving all other text, including Unicode.
/// Recognized sequences consist of `ESC [` followed by zero or more parameter
/// bytes (`0x30..=0x3f`), then zero or more intermediate bytes (`0x20..=0x2f`),
/// and one final byte (`0x40..=0x7e`). This includes SGR color/style sequences
/// and other syntactically valid CSI controls, regardless of their meaning.
///
/// If an escape does not begin a complete valid CSI sequence, the escape is
/// preserved and scanning resumes at the following character. Thus truncated
/// or malformed CSI text and non-CSI escapes are retained, while later valid
/// CSI sequences are still removed. OSC sequences and the single-character
/// C1 CSI control (`U+009B`) are not interpreted.
///
/// ```
/// use clientele::strip_ansi;
///
/// assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
/// assert_eq!(strip_ansi("a\x1bb"), "a\x1bb");
/// assert_eq!(strip_ansi("text\x1b[31"), "text\x1b[31");
/// ```
pub fn strip_ansi(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    if !input.contains('\x1b') {
        return input.to_string();
    }
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Only commit the lookahead once a complete CSI sequence is found.
            // Invalid input is scanned at most twice, preserving linear time.
            let mut sequence = chars.clone();
            if sequence.next() == Some('[') {
                while sequence.next_if(|c| ('0'..='?').contains(c)).is_some() {}
                while sequence.next_if(|c| (' '..='/').contains(c)).is_some() {}
                if sequence.next_if(|c| ('@'..='~').contains(c)).is_some() {
                    chars = sequence;
                    continue;
                }
            }
        }
        output.push(c);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::strip_ansi;

    #[test]
    fn preserves_ordinary_unicode_and_whitespace() {
        for input in ["", "plain text", "café 工具 🦀", "e\u{301}\t\n\r", "a\0b"] {
            assert_eq!(strip_ansi(input), input);
        }
    }

    #[test]
    fn strips_complete_csi_sequences() {
        for (input, expected) in [
            ("\x1b[31mred\x1b[0m", "red"),
            ("\x1b[1;38;2;255;0;128m工具 🦀\x1b[m", "工具 🦀"),
            ("a\x1b[?25lb\x1b[?25hc", "abc"),
            ("a\x1b[2J\x1b[H\x1b[0 qz", "az"),
            ("\x1b[38:2::255:0:0mred\x1b[0m", "red"),
            ("a\x1b[@b\x1b[~c\x1b[ !~d", "abcd"),
        ] {
            assert_eq!(strip_ansi(input), expected, "{input:?}");
        }
    }

    #[test]
    fn preserves_truncated_and_malformed_csi_text() {
        for input in [
            "a\x1b",
            "a\x1b[",
            "a\x1b[31;",
            "a\x1b[0 ",
            "a\x1b[31\nb",
            "a\x1b[31é工具",
            "a\x1b[1 2m",
            "a\x1b[\x7fb",
        ] {
            assert_eq!(strip_ansi(input), input, "{input:?}");
        }
    }

    #[test]
    fn preserves_non_csi_escapes_without_consuming_following_text() {
        for input in [
            "a\x1bb",
            "a\x1bé工具",
            "a\x1b\x1bb",
            "a\x1b7b\x1b8c",
            "a\u{009b}31mb",
            "a\x1b]title\x07b",
            "a\x1b]title\x1b\\b",
        ] {
            assert_eq!(strip_ansi(input), input, "{input:?}");
        }
    }

    #[test]
    fn resumes_stripping_after_malformed_or_non_csi_escapes() {
        for (input, expected) in [
            ("a\x1b\x1b[31mb\x1b[0m", "a\x1bb"),
            ("a\x1b[31\x1b[1mb\x1b[0m", "a\x1b[31b"),
            ("a\x1b[\n\x1b[32m文", "a\x1b[\n文"),
            ("a\x1bb\x1b[0mc", "a\x1bbc"),
        ] {
            assert_eq!(strip_ansi(input), expected, "{input:?}");
        }
    }
}
