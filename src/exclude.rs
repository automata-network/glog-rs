use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<ThreadTag> = RefCell::new(ThreadTag::default());
}

#[derive(Default, Debug, Clone)]
pub struct ThreadTag(&'static str);

#[must_use]
pub struct ContextGuard {
    old: ThreadTag,
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        ThreadTag::local(|n| *n = self.old.clone());
    }
}

impl ThreadTag {
    pub fn local<F, E>(f: F) -> E
    where
        F: FnOnce(&mut Self) -> E,
    {
        CONTEXT.with(|n| f(&mut n.borrow_mut()))
    }

    pub fn clone_local() -> Self {
        Self::local(|n| n.clone())
    }

    pub fn tmp_mut<F>(f: F) -> ContextGuard
    where
        F: FnOnce(&mut Self),
    {
        let old = Self::clone_local();
        Self::local(f);
        ContextGuard { old }
    }
}

pub fn set_tag(tag: &'static str) -> ContextGuard {
    ThreadTag::tmp_mut(|n| n.0 = tag)
}

pub fn has_tag(tag: &str) -> bool {
    ThreadTag::local(|n| n.0 == tag)
}

pub fn should_log(tag: &str) -> bool {
    !has_tag(tag) || crate::is_enable(tag)
}
