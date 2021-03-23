//! Traits used to make construction of bytecode builders much easier. These
//! may use complex or generally irritating generics, but it is much better
//! than repeating a lot of boilerplate. It is recommended that you read the
//! documentation before implementing any of these traits.

/// A trait used for "entries" of a bytecode [`Parent`], similar to a
/// [`std::collections::hash_map::Entry`].
pub trait Entry {
    /// The parent type of this entry.
    type Parent: Parent<<Self as Entry>::Element>;
    /// The element type of this entry.
    type Element: Element;

    /// Get the bytecode element behind this entry, if finished.
    ///
    /// # Implementors
    ///
    /// Do not re-implement this function. It has an implementation that is
    /// already sufficient for generalized functionality.
    fn get<'p>(&self, parent: &'p <Self as Entry>::Parent) -> Option<&'p <Self as Entry>::Element> {
        if !self.is_finished(parent) {
            panic!("cannot get an unfinished bytecode element");
        }
        parent.get(self.id())
    }

    /// Check if the bytecode element has been finished.
    ///
    /// # Implementors
    ///
    /// Do not re-implement this function. It has an implementation that is
    /// already sufficient for generalized functionality.
    fn is_finished(&self, parent: &<Self as Entry>::Parent) -> bool {
        parent.is_finished(self.id())
    }

    /// Enter the builder context and use the closure to build the bytecode
    /// element.
    ///
    /// # Implementors
    ///
    /// Do not re-implement this function. It has an implementation that is
    /// already sufficient for generalized functionality.
    fn enter(
        &mut self,
        parent: &mut <Self as Entry>::Parent,
        f: impl FnOnce(
            <<Self as Entry>::Element as Element>::Builder,
        ) -> <<Self as Entry>::Element as Element>::Builder,
    ) -> &mut Self {
        if self.is_finished(parent) {
            panic!("cannot enter a finished bytecode element");
        }
        let builder = f(parent.create_builder(self.id()));
        self.internal_build(parent, builder);
        self
    }

    /// Finish the bytecode element.
    ///
    /// # Implementors
    ///
    /// Do not re-implement this function. It has an implementation that is
    /// already sufficient for generalized functionality.
    fn finish<'p>(
        &mut self,
        parent: &'p mut <Self as Entry>::Parent,
    ) -> &'p <Self as Entry>::Element {
        if self.is_finished(parent) {
            panic!("cannot build a finished bytecode element")
        }
        parent.finish(self.id())
    }

    /// Get the ID of the bytecode element behind the entry.
    fn id(&self) -> <<Self as Entry>::Element as Element>::Id;

    /// An internal function to register changes caused by a builder. Do not
    /// call this function as it may cause errors.
    fn internal_build(
        &mut self,
        parent: &mut <Self as Entry>::Parent,
        builder: <<Self as Entry>::Element as Element>::Builder,
    );
}

/// A bytecode parent, i.e. a structure that contains one or more types of
/// bytecode elements.
pub trait Parent<Child>
where
    Child: Element,
{
    /// Check if a bytecode element is finished. This function should return
    /// `false` if the ID was not present within the parent. This function is
    /// called by [`Entry::is_finished`].
    fn is_finished(&self, id: Child::Id) -> bool;

    /// Get a reference to a bytecode element. This function is called by
    /// [`Entry::get`].
    fn get(&self, id: Child::Id) -> Option<&Child>;

    /// Create a builder from the specified bytecode element ID.
    ///
    /// # Panics
    ///
    /// This function may panic if the ID was not present within the parent,
    /// depending on the implementor.
    fn create_builder(&mut self, id: Child::Id) -> Child::Builder;

    /// Finish the bytecode element with the specified ID.
    ///
    /// # Panics
    ///
    /// This function may panic if the ID was not present within the parent,
    /// depending on the implementor.
    fn finish(&mut self, id: Child::Id) -> &Child;
}

/// A bytecode element, i.e. a single element in the bytecode, whether
/// high-level or low-level.
///
/// For example, all these are bytecode elements:
/// - Modules
/// - Functions
/// - Blocks
pub trait Element {
    /// The entry type of this bytecode element.
    type Entry: Entry;
    /// The builder type of this bytecode element, i.e. a structure that can
    /// construct a bytecode element. It should store changes within its own
    /// structure, then its corresponding [`Entry`] type should implement the
    /// [`Entry::internal_build`] function that writes these changes to the
    /// [`Parent`].
    type Builder;
    /// The ID type of this bytecode element. It is required to be [`Copy`] to
    /// ensure that IDs are small and also because of limitations with
    /// borrowing and traits.
    type Id: PartialEq + Eq + Copy;
}
