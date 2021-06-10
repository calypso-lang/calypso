use std::convert::AsRef;
use std::fmt::{self, Debug, Display};

use lasso::{Key, Spur, ThreadedRodeo};
use once_cell::sync::OnceCell;

pub use lasso;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Symbol {
    #[must_use]
    pub fn intern(string: &str) -> Self {
        Self(get_interner().get_or_intern(string))
    }

    #[must_use]
    pub fn intern_static(string: &'static str) -> Self {
        Self(get_interner().get_or_intern_static(string))
    }

    fn intern_static_2(string: &'static str) -> Self {
        Self(
            GLOBAL_INTERNER
                .get_or_init(ThreadedRodeo::new)
                .get_or_intern_static(string),
        )
    }

    #[must_use]
    // we know that `Spur` is 32 bits
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_u32(self) -> u32 {
        self.0.into_usize() as u32
    }

    #[must_use]
    pub fn as_str(self) -> &'static str {
        get_interner().resolve(&self.0)
    }

    #[must_use]
    pub fn is_empty(self) -> bool {
        self == kw::EMPTY
    }

    #[must_use]
    pub fn is_keyword(self) -> bool {
        self == kw::TRUE || self == kw::FALSE
    }
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PotentiallyInterned<'a> {
    Uninterned(&'a str),
    Interned(Symbol),
}

impl<'a> Debug for PotentiallyInterned<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'a> Display for PotentiallyInterned<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'a> PotentiallyInterned<'a> {
    #[must_use]
    pub fn potentially_intern(string: &'a str) -> Self {
        if string.len() > 255 {
            Self::Uninterned(string)
        } else {
            Self::Interned(Symbol::intern(string))
        }
    }
}

impl<'a> AsRef<str> for PotentiallyInterned<'a> {
    fn as_ref(&self) -> &str {
        match self {
            PotentiallyInterned::Interned(sym) => sym.as_str(),
            PotentiallyInterned::Uninterned(s) => *s,
        }
    }
}

// I am aware global state is bad and whatnot, but this is the only way I know
// of to have all this data live for `'static` without plain leaking memory.
//
// Also it's easy. Ish.
static GLOBAL_INTERNER: OnceCell<ThreadedRodeo> = OnceCell::new();

pub fn get_interner() -> &'static ThreadedRodeo {
    let int = GLOBAL_INTERNER.get_or_init(ThreadedRodeo::new);
    kw::init();
    int
}

macro_rules! intern_static {
    ($mod:ident, $name:ident => {$($enum_ident:ident; $static_ident:ident: $str:expr),*$(,)?}) => {
        #[allow(dead_code)]
        pub mod $mod {
            ::lazy_static::lazy_static! {
                $(
                    pub static ref $static_ident: $crate::symbol::Symbol
                        = $crate::symbol::Symbol::intern_static_2($str);
                )*
            }

            pub(super) fn init() {
                $(::lazy_static::initialize(&$static_ident);)*
            }

            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub enum $name {
                $(
                    $enum_ident,
                )*
            }

            impl From<$crate::symbol::Symbol> for $name {
                fn from(sym: $crate::symbol::Symbol) -> Self {
                    $(
                        if sym == $static_ident {
                            return Self::$enum_ident;
                        }
                    )*
                    unreachable!()
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

            // Not sure why, but I have to use `.deref()` like this,
            // otherwise it infinitely recurses. :/
            use ::std::ops::Deref;
            $(
                impl PartialEq<$static_ident> for $crate::symbol::Symbol {
                    fn eq(&self, symbol: &$static_ident) -> bool {
                        self == symbol.deref()
                    }
                }
                impl PartialEq<$static_ident> for $static_ident {
                    fn eq(&self, symbol: &$static_ident) -> bool {
                        self.deref() == symbol.deref()
                    }
                }
                impl Eq for $static_ident {}
            )*
        }

    }
}

intern_static! {kw, Keyword => {
    Empty; EMPTY: "",
    Under; UNDERSCORE: "_",

    True; TRUE: "true",
    False; FALSE: "false"
}}
