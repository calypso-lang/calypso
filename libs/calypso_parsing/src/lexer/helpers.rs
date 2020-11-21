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

#[inline]
pub(super) fn is_valid_for_char_literal(ch: char) -> bool {
    !ch.is_control()
}

#[inline]
pub(super) fn is_whitespace(elem: &Spanned<char>) -> bool {
    WHITESPACE.contains(elem.value())
}

#[inline]
pub(super) fn is_ident_start(elem: &Spanned<char>) -> bool {
    elem.value_owned().is_ascii_alphabetic() || elem.value_owned() == '_'
}

#[inline]
pub(super) fn is_ident_continue(elem: &Spanned<char>) -> bool {
    is_ident_start(elem) || elem.value_owned().is_ascii_digit()
}

use calypso_base::span::Spanned;
