//! Composable cache type provides caching in the graph nodes.
use std::{cell::RefCell};

/// Cache is a simple abstraction that store Copy type, that allow one to get previously computed value.
/// If Cache is already set and valid, then it returns stored value,
/// otherwise it compute new value form provided Fn.
#[derive(Default)]
pub struct Cache<T> {
    val: RefCell<Option<T>>
}

impl<T: Copy> Cache<T> {
    pub(crate) fn new() -> Self {
        Self { val: RefCell::new(None) }
    }

    /// If cache is valid, then return previusly stored value. Otherwise compute new value with `f` and store it.
    pub(crate) fn get_or_else(&self, f: impl Fn() -> T) -> T {
        *self.val.borrow_mut().get_or_insert_with(f)
    }

    pub(crate) fn get(&self) -> Option<T> {
        *self.val.borrow()
    }

    /// Invalidate cache so that susequent request to it will lead to recomputations.
    pub(crate) fn invalidate(&self) {
        self.val.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_set() {
        let cache = Cache::new();
        assert!(cache.get().is_none());
        assert_eq!(cache.get_or_else(|| 3.0), 3.0);
        assert_eq!(cache.get(), Some(3.0));
    }

    #[test]
    fn test_cache_invalidation() {
        let cache = Cache::new();
        cache.get_or_else(|| 5.0);
        cache.invalidate();
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_cache_update() {
        let cache = Cache::new();
        assert!(cache.get().is_none());
        cache.get_or_else(|| 25.0);
        assert_eq!(cache.get(), Some(25.0));
        assert_eq!(cache.get_or_else(|| 0.0), 25.0);
        cache.invalidate();
        assert_eq!(cache.get_or_else(|| -5.0), -5.0); 
    }
}