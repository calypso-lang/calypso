use calypso_base::static_list as sl;

sl!(WHITESPACE: char = [
    '\t',       // Horizontal tab
    '\n',       // Line feed
    '\u{000B}', // Vertical tab
    '\u{000C}', // Form feed
    '\r',       // Carriage return
    ' ',        // Space
    '\u{0085}', // Next line
    '\u{200E}', // Left-to-right mark
    '\u{200F}', // Right-to-left mark
    '\u{2028}', // Line separator
    '\u{2029}', // Paragraph separator
]);

sl!(INVALID_FOR_CHAR_LITERAL: char = ['\n', '\r', '\0',]);

pub(super) fn is_valid_for_char_literal(ch: char) -> bool {
    !INVALID_FOR_CHAR_LITERAL.contains(&ch) && !ch.is_control()
}

pub(super) fn is_whitespace(ch: char) -> bool {
    WHITESPACE.contains(&ch)
}

pub(super) fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

pub(super) fn is_ident_continue(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}
