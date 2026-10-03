// This is free and unencumbered software released into the public domain.

/// Strips ANSI CSI sequences and OSC 8 hyperlink controls, retaining visible text.
///
/// Available without optional features. Returns an owned string with complete
/// CSI sequences and supported hyperlink controls removed, preserving all other
/// text, including Unicode. Recognized CSI sequences (including those emitted by
/// `color_print`) consist of `ESC [` followed by zero or more parameter
/// bytes (`0x30..=0x3f`), then zero or more intermediate bytes (`0x20..=0x2f`),
/// and one final byte (`0x40..=0x7e`). This includes SGR color/style sequences
/// and other syntactically valid CSI controls, regardless of their meaning.
///
/// Hyperlink controls have the form `ESC ] 8 ; params ; URI` terminated by BEL
/// (`U+0007`) or ST (ESC followed by a backslash). Parameters and URI are opaque
/// text without control characters; either may be empty, and the URI may contain
/// semicolons. Opening and closing controls are removed independently, without a
/// matched pair. The label between them is retained, with any CSI styling stripped.
/// Other OSC commands and single-character C1 controls are not interpreted.
///
/// If an escape does not begin a complete supported sequence, the escape is
/// preserved and scanning resumes at the following character. Thus truncated
/// or malformed CSI/OSC text and unsupported escapes are retained, while later
/// supported sequences are still removed. An OSC 8 control with a missing field
/// separator, an embedded control character, or a missing terminator is malformed.
///
/// ```
/// use clientele::strip_ansi;
///
/// assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
/// assert_eq!(strip_ansi("a\x1bb"), "a\x1bb");
/// assert_eq!(strip_ansi("text\x1b[31"), "text\x1b[31");
/// assert_eq!(
///     strip_ansi("\x1b]8;;https://example.com\x1b\\label\x1b]8;;\x1b\\"),
///     "label"
/// );
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
            // Only commit the lookahead once a complete supported sequence is found.
            // Invalid input is scanned at most twice, preserving linear time.
            let mut sequence = chars.clone();
            let introducer = sequence.next();
            if introducer == Some('[') {
                while sequence.next_if(|c| ('0'..='?').contains(c)).is_some() {}
                while sequence.next_if(|c| (' '..='/').contains(c)).is_some() {}
                if sequence.next_if(|c| ('@'..='~').contains(c)).is_some() {
                    chars = sequence;
                    continue;
                }
            } else if introducer == Some(']') && skip_osc_hyperlink(&mut sequence) {
                chars = sequence;
                continue;
            }
        }
        output.push(c);
    }
    output
}

// Called just after ESC ]. Reject embedded controls promptly so a malformed
// sequence cannot swallow a later escape or cause repeated scans of the suffix.
fn skip_osc_hyperlink(chars: &mut core::iter::Peekable<core::str::Chars<'_>>) -> bool {
    if chars.next() != Some('8') || chars.next() != Some(';') {
        return false;
    }
    let mut has_uri = false;
    while let Some(c) = chars.next() {
        match c {
            '\x07' => return has_uri,
            '\x1b' => return has_uri && chars.next() == Some('\\'),
            ';' => has_uri = true,
            c if c.is_control() => return false,
            _ => {}
        }
    }
    false
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

    #[test]
    fn strips_hyperlinks_with_bel_and_st_terminators() {
        for open_end in ["\x07", "\x1b\\"] {
            for close_end in ["\x07", "\x1b\\"] {
                for params in ["", "id=link:custom=value"] {
                    let input = format!(
                        "before \x1b]8;{params};https://example.com/工具;a=b{open_end}\x1b[31m工具 🦀\x1b[0m\x1b]8;;{close_end} after"
                    );
                    assert_eq!(strip_ansi(input), "before 工具 🦀 after");
                }
            }
        }
    }

    #[test]
    fn strips_independent_and_adjacent_hyperlink_controls() {
        for (input, expected) in [
            ("\x1b]8;;url\x07label", "label"),
            ("label\x1b]8;;\x07", "label"),
            ("\x1b]8;;url\x07\x1b]8;;\x1b\\", ""),
            ("\x1b]8;;one\x07a\x1b]8;;two\x07b\x1b]8;;\x07", "ab"),
        ] {
            assert_eq!(strip_ansi(input), expected, "{input:?}");
        }
    }

    #[test]
    fn preserves_malformed_hyperlinks_and_unsupported_osc_commands() {
        for input in [
            "\x1b]8",
            "\x1b]8;",
            "\x1b]8;;url",
            "\x1b]8;;url\x1b",
            "\x1b]8;;url\x1bX",
            "\x1b]8;url\x07",
            "\x1b]8;url\x1b\\",
            "\x1b]8;id=bad\n;;url\x07",
            "\x1b]8;;bad\turl\x07",
            "\x1b]8;;bad\x7furl\x07",
            "\x1b]8;;url\u{009c}",
            "\x1b]0;title\x07",
            "\x1b]88;;url\x07",
            "\x1b]52;c;data\x1b\\",
            "\u{009d}8;;url\x07",
        ] {
            assert_eq!(strip_ansi(input), input, "{input:?}");
        }
    }

    #[test]
    fn resumes_stripping_after_malformed_hyperlinks() {
        assert_eq!(
            strip_ansi("a\x1b]8;;bad\x1b[31mred\x1b[0m"),
            "a\x1b]8;;badred"
        );
        assert_eq!(
            strip_ansi("\x1b]8;;bad\x1b]8;;good\x07label\x1b]8;;\x07"),
            "\x1b]8;;badlabel"
        );
    }
}
