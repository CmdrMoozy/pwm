use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct Lazy<'a, T> {
    state: UnsafeCell<LazyState<'a, T>>,
}

impl<'a, T> Lazy<'a, T> {
    pub fn new<F: 'a + FnOnce() -> T>(f: F) -> Lazy<'a, T> {
        Lazy { state: UnsafeCell::new(LazyState::Unevaluated(Factory::new(f))) }
    }


    fn force(&self) {
        unsafe {
            match *self.state.get() {
                LazyState::Evaluated(_) => return,
                LazyState::Evaluating => unreachable!(),
                LazyState::Unevaluated(_) => (),
            };

            match ptr::replace(self.state.get(), LazyState::Evaluating) {
                LazyState::Unevaluated(factory) => {
                    *self.state.get() = LazyState::Evaluated(factory.build())
                },
                _ => unreachable!(),
            };
        }
    }

    pub fn into_inner(self) -> T {
        self.force();
        unsafe {
            match self.state.into_inner() {
                LazyState::Evaluated(val) => val,
                _ => unreachable!(),
            }
        }
    }
}

struct Factory<'a, T> {
    f: Box<FnMut() -> T + 'a>,
}

impl<'a, T> Factory<'a, T> {
    fn new<F: 'a + FnOnce() -> T>(f: F) -> Factory<'a, T> {
        // TODO: This is a gross hack to work around the fact that Box<FnOnce> doesn't
        // work. Remove this if Box<FnOnce> ever works on Rust stable, or if FnBox is
        // stabilized.
        let mut f = Some(f);
        Factory { f: Box::new(move || -> T { f.take().unwrap()() }) }
    }

    fn build(mut self) -> T { (self.f)() }
}

enum LazyState<'a, T> {
    Evaluated(T),
    Evaluating,
    Unevaluated(Factory<'a, T>),
}

impl<'x, T> Deref for Lazy<'x, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        self.force();
        match unsafe { &*self.state.get() } {
            &LazyState::Evaluated(ref val) => val,
            _ => unreachable!(),
        }
    }
}

impl<'x, T> DerefMut for Lazy<'x, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        self.force();
        match unsafe { &mut *self.state.get() } {
            &mut LazyState::Evaluated(ref mut val) => val,
            _ => unreachable!(),
        }
    }
}
