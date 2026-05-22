use std::fmt::{self, Debug, Display};
use std::sync::OnceLock;

use lasso::{Key, Spur, ThreadedRodeo};

use crate::syntax::span::Span;

/// An interned string.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Symbol {
    /// Intern a string and return the symbol.
    #[must_use]
    pub fn intern(string: &str) -> Self {
        Self(get_interner().get_or_intern(string))
    }

    /// Intern a static string and return the symbol.
    #[must_use]
    pub fn intern_static(string: &'static str) -> Self {
        Self(get_interner().get_or_intern_static(string))
    }

    #[must_use]
    #[doc(hidden)]
    pub fn _intern_static_inner(string: &'static str) -> Self {
        Self(
            GLOBAL_INTERNER
                .get_or_init(ThreadedRodeo::new)
                .get_or_intern_static(string),
        )
    }

    /// Get the raw index of a symbol.
    #[must_use]
    // we know that `Spur` is 32 bits
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_u32(self) -> u32 {
        self.0.into_usize() as u32
    }

    /// Resolve a symbol to a static string.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        get_interner().resolve(&self.0)
    }

    /// Check if a symbol is the empty string.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self == *special::EMPTY
    }

    // /// Check if a symbol is a keyword.
    // #[must_use]
    // pub fn is_keyword(self) -> bool {
    //     kw::is(self)
    // }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// An identifier, i.e. a combination of a symbol and a span.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    /// The symbol associated with this identifier
    pub symbol: Symbol,
    /// The span associated with this identifier
    pub span: Span,
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:?}", self.symbol, self.span)
    }
}

impl std::ops::Deref for Ident {
    type Target = Symbol;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

// I am aware global state is bad and whatnot, but this is the only way I know
// of to have all this data live for `'static` without plain leaking memory.
//
// Also it's easy. Ish.
static GLOBAL_INTERNER: OnceLock<ThreadedRodeo> = OnceLock::new();

/// Get the global interner.
pub fn get_interner() -> &'static ThreadedRodeo {
    let int = GLOBAL_INTERNER.get_or_init(ThreadedRodeo::new);
    kw::init();
    prim_ty::init();
    special::init();
    int
}

macro_rules! intern_static {
    ($mod:ident, $mod_doc:expr, $name:ident => {$($enum_ident:ident; $static_ident:ident: $str:expr),*$(,)?}) => {
        #[doc = $mod_doc]
        #[allow(dead_code)]
        pub mod $mod {
            $(
                pub static $static_ident: ::std::sync::LazyLock<$crate::symbol::Symbol>
                    = ::std::sync::LazyLock::new(|| $crate::symbol::Symbol::_intern_static_inner($str));
            )*

            /// Initialize all of the symbols in this module.
            pub(super) fn init() {
                $(::std::sync::LazyLock::force(&$static_ident);)*
            }

            /// An enum containing all of the statically interned values in this module.
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub enum $name {
                $(
                    $enum_ident,
                )*
            }

            impl ::std::convert::TryFrom<$crate::symbol::Symbol> for $name {
                type Error = $crate::symbol::Symbol;

                fn try_from(sym: $crate::symbol::Symbol) -> Result<Self, Self::Error> {
                    $(
                        if sym == *$static_ident {
                            return Ok(Self::$enum_ident);
                        }
                    )*
                    return Err(sym);
                }
            }
            impl From<$name> for $crate::symbol::Symbol {
                fn from(elem: $name) -> Self {
                    $(
                        if elem == $name::$enum_ident {
                            return *$static_ident;
                        }
                    )*
                    unreachable!()
                }
            }
	    impl $name {
		pub fn description(self) -> &'static str {
		    $(
			if self == $name::$enum_ident {
			    return concat!("`", $str, "`");
			}
		    )*
		    unreachable!()
		}
	    }

            /// Check if the given symbol is one of the statically interned
            /// members in this module.
            #[must_use]
            pub fn is(sym: $crate::symbol::Symbol) -> bool {
                $(
                    sym == $crate::symbol::Symbol::from($name::$enum_ident)
                )||*
            }
        }

    }
}

intern_static! {kw, "Keywords", Keyword => {
    True; TRUE: "true",
    False; FALSE: "false",
    Let; LET: "let",
    Do; DO: "do",
    End; END: "end",
    In; IN: "in",
    Fn; FN: "fn",
    If; IF: "if",
    Then; THEN: "then",
    Else; ELSE: "else",
    Pub; PUB: "pub",
    Ref; REF: "ref",
    Mut; MUT: "mut",
    Shared; SHARED: "shared",
    Imm; IMM: "imm",
    Unique; UNIQUE: "unique",
    Type; TYPE: "type",
}}

intern_static! {prim_ty, "Simple primitive types (no generics)", PrimitiveTy => {
    UInt; UINT: "UInt",
    UInt8; UINT8: "UInt8",
    UInt16; UINT16: "UInt16",
    UInt32; UINT32: "UInt32",
    UInt64; UINT64: "UInt64",
    UIntPtr; UINTPTR: "UIntPtr",
    Int; INT: "Int",
    Int8; INT8: "Int8",
    Int16; INT16: "Int16",
    Int32; INT32: "Int32",
    Int64; INT64: "Int64",
    IntPtr; INTPTR: "IntPtr",
    String; STRING: "String",
    Bool; BOOL: "Bool",
}}

intern_static! {misc_ty, "Miscellaneous types", MiscTy => {
    Array; ARRAY: "Array",
    Option; OPTION: "Option",
}}

intern_static! {special, "Special strings", Special => {
    Empty; EMPTY: "",
}}
