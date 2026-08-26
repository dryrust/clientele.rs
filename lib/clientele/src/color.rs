// This is free and unencumbered software released into the public domain.

/// Strips the ANSI CSI escape sequences emitted by `color_print`.
pub fn strip_ansi(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    if !input.contains('\x1b') {
        return input.to_string();
    }
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.next() == Some('[') {
                for c in chars.by_ref() {
                    if ('@'..='~').contains(&c) {
                        break;
                    }
                }
            }
        } else {
            output.push(c);
        }
    }
    output
}
