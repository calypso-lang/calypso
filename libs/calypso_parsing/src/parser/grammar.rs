// auto-generated: "lalrpop 0.19.5"
// sha3: 7b56d2d7c9f9ad183308998d247dcf2487ec54661f28ff77cb587bb43527668
use calypso_diagnostic::prelude::*;
use calypso_ast::expr::{Expr, Primary, BinOpKind, UnOpKind};
use crate::parser::tokens::Tok;
use crate::lexer::{TokenType, Keyword};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use calypso_diagnostic::prelude::*;
    use calypso_ast::expr::{Expr, Primary, BinOpKind, UnOpKind};
    use crate::parser::tokens::Tok;
    use crate::lexer::{TokenType, Keyword};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(Tok<'input>),
        Variant1(f64),
        Variant2(&'input str),
        Variant3(i64),
        Variant4(u64),
        Variant5(Box<Expr<'input>>),
        Variant6(BinOpKind),
    }
    const __ACTION: &[i8] = &[
        // State 0
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 1
        0, -12, 0, -12, -12, 0, -12, 0, 0, 59, 60, 0, 0, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, -12, 0, 0, -12, -12,
        // State 2
        0, 0, 0, 61, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, -14,
        // State 3
        0, 0, 0, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, -20,
        // State 4
        0, -16, 0, -16, -16, 0, -16, 0, 0, 0, 0, 0, 0, -16, 63, -16, -16, -16, -16, 64, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16,
        // State 5
        0, 0, 0, 0, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, -10, -10,
        // State 6
        0, -18, 0, -18, -18, 0, -18, 0, 0, 0, 0, 0, 0, 66, 0, 67, -18, 68, 69, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18,
        // State 7
        0, 70, 0, -8, -8, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, -8,
        // State 8
        0, 0, 0, 0, 72, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22,
        // State 9
        0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73,
        // State 10
        0, -66, 74, -66, -66, 0, -66, 75, 0, -66, -66, 76, 0, -66, -66, -66, -66, -66, -66, -66, 0, 0, 0, 0, 0, -66, 0, 0, -66, -66,
        // State 11
        0, -26, -26, -26, -26, 0, -26, -26, 77, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, -26, 0, 0, -26, -26,
        // State 12
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 13
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 14
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 15
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 16
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 17
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 18
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 19
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 20
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 21
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 22
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 23
        13, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 54, 0, 55, 0, 56, 0, 57, 58, 0, 0,
        // State 26
        0, 70, 0, -9, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, -9,
        // State 27
        0, 0, 0, 0, -11, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, -11, -11,
        // State 28
        0, -13, 0, -13, -13, 0, -13, 0, 0, 59, 60, 0, 0, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, -13, 0, 0, -13, -13,
        // State 29
        0, 0, 0, 61, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15,
        // State 30
        0, -17, 0, -17, -17, 0, -17, 0, 0, 0, 0, 0, 0, -17, 63, -17, -17, -17, -17, 64, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17,
        // State 31
        0, -19, 0, -19, -19, 0, -19, 0, 0, 0, 0, 0, 0, 66, 0, 67, -19, 68, 69, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, -19,
        // State 32
        0, 0, 0, 0, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, -21,
        // State 33
        0, 0, 0, 0, 72, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23,
        // State 34
        0, -63, -63, -63, -63, 0, -63, -63, -63, -63, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, 0, 0, 0, 0, 0, -63, 0, 0, -63, -63,
        // State 35
        0, -1, 0, -1, -1, 0, -1, 0, 0, -1, -1, 0, 0, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, -1, 0, 0, -1, -1,
        // State 36
        0, 0, 0, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, -28, -28,
        // State 37
        0, 0, 0, 0, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 38
        0, -32, 0, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, -32, 0, 0, -32, -32,
        // State 39
        0, 0, 0, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, -35,
        // State 40
        0, -39, 0, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, 0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, -39, 0, 0, -39, -39,
        // State 41
        0, -44, 0, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, -44, -44,
        // State 42
        0, 0, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48,
        // State 43
        0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50,
        // State 44
        0, -52, -52, -52, -52, 0, -52, -52, 0, -52, -52, -52, 0, -52, -52, -52, -52, -52, -52, -52, 0, 0, 0, 0, 0, -52, 0, 0, -52, -52,
        // State 45
        0, -59, -59, -59, -59, 0, -59, -59, 0, -59, -59, -59, 0, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, -59, 0, 0, -59, -59,
        // State 46
        0, -62, -62, -62, -62, 0, -62, -62, -62, -62, -62, -62, 0, -62, -62, -62, -62, -62, -62, -62, 0, 0, 0, 0, 0, -62, 0, 0, -62, -62,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, -61, -61, -61, -61, 0, -61, -61, -61, -61, -61, -61, 0, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, -61, 0, 0, -61, -61,
        // State 49
        0, -24, -24, -24, -24, 0, -24, -24, 0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, -24, 0, 0, -24, -24,
        // State 50
        0, -64, -64, -64, -64, 0, -64, -64, -64, -64, -64, -64, 0, -64, -64, -64, -64, -64, -64, -64, 0, 0, 0, 0, 0, -64, 0, 0, -64, -64,
        // State 51
        0, -6, 0, -6, -6, 0, -6, 0, 0, -6, -6, 0, 0, -6, -6, -6, -6, -6, -6, -6, 0, 0, 0, 0, 0, -6, 0, 0, -6, -6,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 82, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -58, -58, -58, -58, 0, -58, -58, -58, -58, -58, -58, 0, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, -58, 0, 0, -58, -58,
        // State 54
        0, -56, -56, -56, -56, 0, -56, -56, -56, -56, -56, -56, 0, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, -56, 0, 0, -56, -56,
        // State 55
        0, -57, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, -57, 0, 0, -57, -57,
        // State 56
        0, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, 0, 0, 0, 0, 0, -38, 0, 0, -38, -38,
        // State 57
        0, -37, -37, -37, -37, 0, -37, -37, -37, -37, -37, -37, 0, -37, -37, -37, -37, -37, -37, -37, 0, 0, 0, 0, 0, -37, 0, 0, -37, -37,
        // State 58
        -2, 0, 0, 0, 0, -2, 0, 0, 0, 0, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, -2, 0, -2, 0, -2, 0, -2, -2, 0, 0,
        // State 59
        -3, 0, 0, 0, 0, -3, 0, 0, 0, 0, -3, 0, -3, 0, 0, 0, 0, 0, 0, 0, -3, 0, -3, 0, -3, 0, -3, -3, 0, 0,
        // State 60
        -29, 0, 0, 0, 0, -29, 0, 0, 0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, -29, 0, -29, 0, -29, -29, 0, 0,
        // State 61
        -31, 0, 0, 0, 0, -31, 0, 0, 0, 0, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, -31, 0, -31, 0, -31, -31, 0, 0,
        // State 62
        -34, 0, 0, 0, 0, -34, 0, 0, 0, 0, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, -34, 0, -34, 0, -34, 0, -34, -34, 0, 0,
        // State 63
        -33, 0, 0, 0, 0, -33, 0, 0, 0, 0, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, -33, 0, -33, 0, -33, -33, 0, 0,
        // State 64
        -36, 0, 0, 0, 0, -36, 0, 0, 0, 0, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, -36, 0, -36, 0, -36, 0, -36, -36, 0, 0,
        // State 65
        -40, 0, 0, 0, 0, -40, 0, 0, 0, 0, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, -40, 0, -40, 0, -40, -40, 0, 0,
        // State 66
        -42, 0, 0, 0, 0, -42, 0, 0, 0, 0, -42, 0, -42, 0, 0, 0, 0, 0, 0, 0, -42, 0, -42, 0, -42, 0, -42, -42, 0, 0,
        // State 67
        -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, -41, 0, -41, 0, 0, 0, 0, 0, 0, 0, -41, 0, -41, 0, -41, 0, -41, -41, 0, 0,
        // State 68
        -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, -43, 0, -43, 0, 0, 0, 0, 0, 0, 0, -43, 0, -43, 0, -43, 0, -43, -43, 0, 0,
        // State 69
        -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, -46, 0, -46, 0, -46, -46, 0, 0,
        // State 70
        -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, -45, 0, -45, 0, -45, -45, 0, 0,
        // State 71
        -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, -49, 0, -49, 0, -49, 0, -49, -49, 0, 0,
        // State 72
        -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, -51, 0, -51, 0, -51, -51, 0, 0,
        // State 73
        0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, -55, 0, -55, 0, -55, 0, -55, -55, 0, 0,
        // State 74
        0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, -53, 0, -53, 0, -53, -53, 0, 0,
        // State 75
        0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, -54, 0, -54, 0, -54, -54, 0, 0,
        // State 76
        0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, -60, 0, -60, 0, -60, -60, 0, 0,
        // State 77
        0, -68, 0, -68, -68, 0, -68, 0, 0, -68, -68, 0, 0, -68, -68, -68, -68, -68, -68, -68, 0, 0, 0, 0, 0, -68, 0, 0, -68, -68,
        // State 78
        0, 0, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, -67, 0, -67, -67, 0, -67, 0, 0, -67, -67, 0, 0, -67, -67, -67, -67, -67, -67, -67, 0, 0, 0, 0, 0, -67, 0, 0, -67, -67,
        // State 80
        0, -4, -4, -4, -4, 0, -4, -4, -4, -4, -4, -4, 0, -4, -4, -4, -4, -4, -4, -4, 0, 0, 0, 0, 0, -4, 0, 0, -4, -4,
        // State 81
        0, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, -5, 0, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0, 0, -5, 0, 0, -5, -5,
        // State 82
        0, -7, 0, -7, -7, 0, -7, 0, 0, -7, -7, 0, 0, -7, -7, -7, -7, -7, -7, -7, 0, 0, 0, 0, 0, -7, 0, 0, -7, -7,
        // State 83
        0, -25, -25, -25, -25, 0, -25, -25, 0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, -25, 0, 0, -25, -25,
        // State 84
        0, -27, -27, -27, -27, 0, -27, -27, 0, -27, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, -27, 0, 0, -27, -27,
        // State 85
        0, -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, -65, 0, 0, 0, 0, 0, -65, 0, 0, -65, -65,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -12,
        // State 2
        -14,
        // State 3
        -20,
        // State 4
        -16,
        // State 5
        -10,
        // State 6
        -18,
        // State 7
        -8,
        // State 8
        -22,
        // State 9
        -47,
        // State 10
        -66,
        // State 11
        -26,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -9,
        // State 27
        -11,
        // State 28
        -13,
        // State 29
        -15,
        // State 30
        -17,
        // State 31
        -19,
        // State 32
        -21,
        // State 33
        -23,
        // State 34
        -63,
        // State 35
        -1,
        // State 36
        -28,
        // State 37
        -30,
        // State 38
        -32,
        // State 39
        -35,
        // State 40
        -39,
        // State 41
        -44,
        // State 42
        -48,
        // State 43
        -50,
        // State 44
        -52,
        // State 45
        -59,
        // State 46
        -62,
        // State 47
        -69,
        // State 48
        -61,
        // State 49
        -24,
        // State 50
        -64,
        // State 51
        -6,
        // State 52
        0,
        // State 53
        -58,
        // State 54
        -56,
        // State 55
        -57,
        // State 56
        -38,
        // State 57
        -37,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        -68,
        // State 78
        0,
        // State 79
        -67,
        // State 80
        -4,
        // State 81
        -5,
        // State 82
        -7,
        // State 83
        -25,
        // State 84
        -27,
        // State 85
        -65,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                18 => 28,
                _ => 1,
            },
            1 => 15,
            2 => 34,
            3 => 35,
            4 => 36,
            5 => 37,
            6 => 38,
            7 => 39,
            8 => 40,
            9 => 41,
            10 => 42,
            11 => 43,
            12 => 44,
            13 => 45,
            14 => match state {
                19 => 29,
                _ => 2,
            },
            15 => 16,
            16 => match state {
                22 => 32,
                _ => 3,
            },
            17 => 17,
            18 => match state {
                20 => 30,
                _ => 4,
            },
            19 => 18,
            20 => match state {
                17 => 27,
                _ => 5,
            },
            21 => 19,
            22 => 46,
            23 => match state {
                21 => 31,
                _ => 6,
            },
            24 => 20,
            25 => match state {
                16 => 26,
                _ => 7,
            },
            26 => 21,
            27 => match state {
                13 => 78,
                _ => 47,
            },
            28 => match state {
                23 => 33,
                _ => 8,
            },
            29 => 22,
            30 => 9,
            31 => 23,
            32 => 10,
            33 => 24,
            34 => 48,
            35 => match state {
                24 => 83,
                25 => 84,
                _ => 49,
            },
            36 => 25,
            37 => 50,
            38 => 11,
            39 => match state {
                12 => 77,
                14 => 79,
                15 => 82,
                _ => 51,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""**""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###"">>""###,
            r###""FloatLit""###,
            r###""Ident""###,
            r###""SintLit""###,
            r###""StringLit""###,
            r###""UintLit""###,
            r###""^""###,
            r###""false""###,
            r###""true""###,
            r###""|""###,
            r###""||""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        source_id: usize,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = CalError;
        type Token = Tok<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Expr<'input>>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.source_id,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Tok<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Tok::Unprocessed(TokenType::Bang) if true => Some(0),
            Tok::Unprocessed(TokenType::BangEq) if true => Some(1),
            Tok::Unprocessed(TokenType::Percent) if true => Some(2),
            Tok::Unprocessed(TokenType::And) if true => Some(3),
            Tok::Unprocessed(TokenType::AndAnd) if true => Some(4),
            Tok::Unprocessed(TokenType::LParen) if true => Some(5),
            Tok::Unprocessed(TokenType::RParen) if true => Some(6),
            Tok::Unprocessed(TokenType::Star) if true => Some(7),
            Tok::Unprocessed(TokenType::StarStar) if true => Some(8),
            Tok::Unprocessed(TokenType::Plus) if true => Some(9),
            Tok::Unprocessed(TokenType::Minus) if true => Some(10),
            Tok::Unprocessed(TokenType::Slash) if true => Some(11),
            Tok::Unprocessed(TokenType::Colon) if true => Some(12),
            Tok::Unprocessed(TokenType::Lt) if true => Some(13),
            Tok::Unprocessed(TokenType::LtLt) if true => Some(14),
            Tok::Unprocessed(TokenType::LtEq) if true => Some(15),
            Tok::Unprocessed(TokenType::EqEq) if true => Some(16),
            Tok::Unprocessed(TokenType::Gt) if true => Some(17),
            Tok::Unprocessed(TokenType::GtEq) if true => Some(18),
            Tok::Unprocessed(TokenType::GtGt) if true => Some(19),
            Tok::Float(_) if true => Some(20),
            Tok::Ident(_) if true => Some(21),
            Tok::Sint(_) if true => Some(22),
            Tok::String(_) if true => Some(23),
            Tok::Uint(_) if true => Some(24),
            Tok::Unprocessed(TokenType::Caret) if true => Some(25),
            Tok::Unprocessed(TokenType::Keyword(Keyword::False)) if true => Some(26),
            Tok::Unprocessed(TokenType::Keyword(Keyword::True)) if true => Some(27),
            Tok::Unprocessed(TokenType::Pipe) if true => Some(28),
            Tok::Unprocessed(TokenType::PipePipe) if true => Some(29),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Tok<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 25 | 26 | 27 | 28 | 29 => __Symbol::Variant0(__token),
            20 => match __token {
                Tok::Float(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            21 | 23 => match __token {
                Tok::Ident(__tok0) | Tok::String(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            22 => match __token {
                Tok::Sint(__tok0) if true => __Symbol::Variant3(__tok0),
                _ => unreachable!(),
            },
            24 => match __token {
                Tok::Uint(__tok0) if true => __Symbol::Variant4(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            source_id: usize,
            __tokens0: __TOKENS,
        ) -> Result<Box<Expr<'input>>, __lalrpop_util::ParseError<usize, Tok<'input>, CalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    source_id,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        source_id: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Box<Expr<'input>>,__lalrpop_util::ParseError<usize, Tok<'input>, CalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(source_id, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(source_id, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpKind, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AddExpr = BinOp<AddExpr, AddOp, UnaryExpr> => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AddOp = "+" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AddOp = "-" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Atom = ":", "Ident" => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(source_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Atom = ":", "StringLit" => ActionFn(27);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(source_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<AddExpr, AddOp, UnaryExpr> = UnaryExpr => ActionFn(51);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<AddExpr, AddOp, UnaryExpr> = AddExpr, AddOp, UnaryExpr => ActionFn(52);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitAndExpr, BitAndOp, EqualityExpr> = EqualityExpr => ActionFn(59);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitAndExpr, BitAndOp, EqualityExpr> = BitAndExpr, BitAndOp, EqualityExpr => ActionFn(60);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitOrExpr, BitOrOp, BitXorExpr> = BitXorExpr => ActionFn(63);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitOrExpr, BitOrOp, BitXorExpr> = BitOrExpr, BitOrOp, BitXorExpr => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitShiftExpr, BitShiftOp, AddExpr> = AddExpr => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitShiftExpr, BitShiftOp, AddExpr> = BitShiftExpr, BitShiftOp, AddExpr => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action54::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitXorExpr, BitXorOp, BitAndExpr> = BitAndExpr => ActionFn(61);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<BitXorExpr, BitXorOp, BitAndExpr> = BitXorExpr, BitXorOp, BitAndExpr => ActionFn(62);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action62::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<ComparisonExpr, ComparisonOp, BitShiftExpr> = BitShiftExpr => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<ComparisonExpr, ComparisonOp, BitShiftExpr> = ComparisonExpr, ComparisonOp, BitShiftExpr => ActionFn(56);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action56::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<EqualityExpr, EqualityOp, ComparisonExpr> = ComparisonExpr => ActionFn(57);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<EqualityExpr, EqualityOp, ComparisonExpr> = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(58);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action58::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<LogAndExpr, LogAndOp, BitOrExpr> = BitOrExpr => ActionFn(65);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<LogAndExpr, LogAndOp, BitOrExpr> = LogAndExpr, LogAndOp, BitOrExpr => ActionFn(66);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<LogOrExpr, LogOrOp, LogAndExpr> = LogAndExpr => ActionFn(67);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<LogOrExpr, LogOrOp, LogAndExpr> = LogOrExpr, LogOrOp, LogAndExpr => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action68::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<MulExpr, MulOp, PowExpr> = PowExpr => ActionFn(49);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOp<MulExpr, MulOp, PowExpr> = MulExpr, MulOp, PowExpr => ActionFn(50);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action50::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOpR<Term, PowOp, PowExpr> = Term => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOpR<Term, PowOp, PowExpr> = Term, PowOp, PowExpr => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitAndExpr = BinOp<BitAndExpr, BitAndOp, EqualityExpr> => ActionFn(6);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitAndOp = "&" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitOrExpr = BinOp<BitOrExpr, BitOrOp, BitXorExpr> => ActionFn(4);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitOrOp = "|" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitShiftExpr = BinOp<BitShiftExpr, BitShiftOp, AddExpr> => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitShiftOp = ">>" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitShiftOp = "<<" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitXorExpr = BinOp<BitXorExpr, BitXorOp, BitAndExpr> => ActionFn(5);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitXorOp = "^" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonExpr = BinOp<ComparisonExpr, ComparisonOp, BitShiftExpr> => ActionFn(8);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<=" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">=" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // EqualityExpr = BinOp<EqualityExpr, EqualityOp, ComparisonExpr> => ActionFn(7);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // EqualityOp = "==" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // EqualityOp = "!=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = LogOrExpr => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogAndExpr = BinOp<LogAndExpr, LogAndOp, BitOrExpr> => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogAndOp = "&&" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogOrExpr = BinOp<LogOrExpr, LogOrOp, LogAndExpr> => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogOrOp = "||" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // MulExpr = BinOp<MulExpr, MulOp, PowExpr> => ActionFn(14);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // MulOp = "*" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // MulOp = "/" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // MulOp = "%" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Number = "SintLit" => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Number = "UintLit" => ActionFn(22);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Number = "FloatLit" => ActionFn(23);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PowExpr = BinOpR<Term, PowOp, PowExpr> => ActionFn(15);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PowOp = "**" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Number => ActionFn(18);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Bool => ActionFn(19);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Atom => ActionFn(20);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Primary => ActionFn(16);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(source_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 38)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnaryExpr = MulExpr => ActionFn(11);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(source_id, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnaryExpr = "-", UnaryExpr => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(source_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 39)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        source_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnaryExpr = "!", UnaryExpr => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(source_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 39)
    }
}
pub use self::__parse__Expr::ExprParser;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    source_id: usize,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::UnOp(UnOpKind::Negative, __0))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    source_id: usize,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::UnOp(UnOpKind::UnaryNot, __0))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    source_id: usize,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, i64, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Sint(__0)))
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, u64, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Uint(__0)))
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, f64, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Float(__0)))
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Bool(true)))
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Bool(false)))
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    source_id: usize,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::Atom(__0)))
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    source_id: usize,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::Primary(Primary::AtomStr(__0)))
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::LogicalOr
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::LogicalAnd
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Add
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Subtract
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Multiply
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Divide
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Modulo
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Exponent
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::BitOr
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::BitXor
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::BitAnd
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::BitShiftRight
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::BitShiftLeft
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Equal
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::NotEqual
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Lt
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::Gt
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::LtEq
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> BinOpKind
{
    BinOpKind::GtEq
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    source_id: usize,
    (_, __0, _): (usize, Box<Expr<'input>>, usize),
    (_, __1, _): (usize, BinOpKind, usize),
    (_, __2, _): (usize, Box<Expr<'input>>, usize),
) -> Box<Expr<'input>>
{
    Box::new(Expr::BinOp(__0, __1, __2))
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), __lalrpop_util::ParseError<usize, Tok<'input>, CalError>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Tok<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), __lalrpop_util::ParseError<usize, Tok<'input>, CalError>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Tok<'input>, usize), CalError> {
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), __lalrpop_util::ParseError<usize, Tok<'input>, CalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
