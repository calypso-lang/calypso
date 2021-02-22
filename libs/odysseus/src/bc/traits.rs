pub trait Entry {
    type Parent: Parent<<Self as Entry>::Container>;
    type Container: Container;

    fn get<'p>(
        &self,
        parent: &'p <Self as Entry>::Parent,
    ) -> Option<&'p <Self as Entry>::Container> {
        if !self.is_finished(parent) {
            panic!("cannot get an unfinished bytecode element");
        }
        parent.get(self.id())
    }

    fn is_finished<'p>(&self, parent: &'p <Self as Entry>::Parent) -> bool {
        parent.is_finished(self.id())
    }

    fn enter(
        &mut self,
        parent: &mut <Self as Entry>::Parent,
        f: impl FnOnce(<<Self as Entry>::Container as Container>::Builder),
    ) -> &mut Self {
        if self.is_finished(parent) {
            panic!("cannot enter a finished bytecode element");
        }
        f(parent.create_builder(self.id()));
        self
    }

    fn finish<'p>(
        &mut self,
        parent: &'p mut <Self as Entry>::Parent,
    ) -> &'p <Self as Entry>::Container {
        if self.is_finished(parent) {
            panic!("cannot build a finished bytecode element")
        }
        parent.finish(self.id())
    }

    fn id(&self) -> <<Self as Entry>::Container as Container>::Id;
}

pub trait Parent<Child>
where
    Child: Container,
{
    fn is_finished(&self, id: Child::Id) -> bool;
    fn get(&self, id: Child::Id) -> Option<&Child>;
    fn create_builder(&mut self, id: Child::Id) -> Child::Builder;
    fn finish(&mut self, id: Child::Id) -> &Child;
}

pub trait Container {
    type Entry: Entry;
    type Builder;
    type Id: PartialEq + Eq + Copy;
}
