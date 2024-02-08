mod unify;

#[derive(Clone, Debug)]
pub struct TypeckCtxt {}

impl Default for TypeckCtxt {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ElabError;
