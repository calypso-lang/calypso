use std::fmt::{self, Debug, Display};

use lasso::{Key, Spur, ThreadedRodeo};
use once_cell::sync::OnceCell;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Symbol {
    pub fn intern(string: &str) -> Self {
        Self(get_interner().get_or_intern(string))
    }

    pub fn intern_static(string: &'static str) -> Self {
        Self(get_interner().get_or_intern_static(string))
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into_usize() as u32
    }

    pub fn as_str(&self) -> &'static str {
        get_interner().resolve(&self.0)
    }

    pub fn is_empty(&self) -> bool {
        self == &kw::EMPTY
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

static GLOBAL_INTERNER: OnceCell<ThreadedRodeo> = OnceCell::new();

pub fn get_interner() -> &'static ThreadedRodeo {
    let int = GLOBAL_INTERNER.get_or_init(|| ThreadedRodeo::new());
    kw::init();
    int
}

macro_rules! intern_static {
    ($mod:ident => {$($ident:ident: $str:expr),*$(,)?}) => {
        #[allow(dead_code)]
        mod $mod {
            ::lazy_static::lazy_static! {
                $(
                    pub static ref $ident: $crate::symbol::Symbol
                        = $crate::symbol::Symbol::intern_static($str);
                )*
            }

            pub(super) fn init() {
                $(::lazy_static::initialize(&$ident);)*
            }

            $(
                impl PartialEq<$ident> for $crate::symbol::Symbol {
                    fn eq(&self, symbol: &$ident) -> bool {
                        self == symbol
                    }
                }
                impl PartialEq<$ident> for $ident {
                    fn eq(&self, symbol: &$ident) -> bool {
                        self == symbol
                    }
                }
                impl Eq for $ident {}
            )*
        }

    }
}

intern_static! {kw => {
    EMPTY: "",
    UNDERSCORE: "_",

    TRUE: "true",
    FALSE: "false",
    NULL: "null"
}}
