use calypso_ast::expr::Radix;
use calypso_base::span::Spanned;

static WHITESPACE: &[char] = &[
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
];

#[inline]
pub(super) fn is_valid_for_char_literal(elem: &Spanned<char>) -> bool {
    let ch = elem.value_owned();
    ch != '\n' && ch != '\r' && ch != '\t'
}

#[inline]
pub(super) fn is_whitespace(elem: &Spanned<char>) -> bool {
    is_whitespace_ch(elem.value_owned())
}

#[inline]
pub(super) fn is_whitespace_ch(ch: char) -> bool {
    WHITESPACE.contains(&ch)
}

#[inline]
pub(super) fn is_ident_start(elem: &Spanned<char>) -> bool {
    elem.value().is_ascii_alphabetic() || elem.value() == &'_'
}

#[inline]
pub(super) fn is_ident_continue(elem: &Spanned<char>) -> bool {
    is_ident_start(elem) || elem.value().is_ascii_digit()
}

#[inline]
pub(super) fn is_ident_end(elem: &Spanned<char>) -> bool {
    elem == &'!' || elem == &'?'
}

#[inline]
pub(super) fn is_valid_for(elem: &Spanned<char>, radix: Radix) -> bool {
    let ch = elem.value_owned();
    match radix {
        Radix::None => ch.is_ascii_digit(),
        Radix::Decimal => ch.is_ascii_digit(),
        Radix::Hexadecimal => ch.is_ascii_hexdigit(),
        Radix::Octal => ('0'..'8').contains(&ch),
        Radix::Binary => ch == '0' || ch == '1',
    }
}
