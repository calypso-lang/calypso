use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DiagnosticId(pub u16);

impl From<u16> for DiagnosticId {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

impl DiagnosticId {
    pub fn get_reason(&self) -> Option<&'static str> {
        DIAGNOSTIC_IDS.get(&self).copied()
    }
}

impl DiagnosticId {}

macro_rules! define_diagnostics {
    ($name:ident: {$keyty:ty, $valuety:ty} => { $($id:expr => $value:expr),* }) => {
        lazy_static! {
            static ref $name: HashMap<$keyty, $valuety> = {
              let mut m = HashMap::new();
              $(m.insert(DiagnosticId::from($id), $value));*;
              m
            };
        }
    };
}

define_diagnostics!(DIAGNOSTIC_IDS: {DiagnosticId, &'static str} => {
    0000 => "Diagnostic is not yet implemented. Please file an issue if you experience this in regular usage.",
    0001 => "No corresponding `/*` for `*/`.",
    0002 => "No corresponding `*/` for `/*`.",
    0003 => "Unexpected character."
});
