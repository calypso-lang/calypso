#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    finished: bool,
}

impl Module {
    pub(crate) fn new() -> Self {
        Self { finished: false }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
