// Copyright 2015 Axel Rasmussen
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::ptr;

type StdResult<T, E> = ::std::result::Result<T, E>;

struct Factory<'f, T> {
    f: Box<dyn FnOnce() -> T + 'f>,
}

impl<'f, T> Factory<'f, T> {
    fn new<F: FnOnce() -> T + 'f>(f: F) -> Self {
        Factory { f: Box::new(f) }
    }

    fn build(self) -> T {
        (self.f)()
    }
}

enum LazyState<'f, T> {
    Evaluated(T),
    Evaluating,
    Unevaluated(Factory<'f, T>),
}

pub struct Lazy<'f, T> {
    state: UnsafeCell<LazyState<'f, T>>,
}

impl<'f, T> Lazy<'f, T> {
    pub fn new<F: FnOnce() -> T + 'f>(f: F) -> Self {
        Lazy {
            state: UnsafeCell::new(LazyState::Unevaluated(Factory::new(f))),
        }
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
                }
                _ => unreachable!(),
            };
        }
    }

    pub fn into_inner(self) -> T {
        self.force();
        match self.state.into_inner() {
            LazyState::Evaluated(val) => val,
            _ => unreachable!(),
        }
    }
}

impl<'f, T> Deref for Lazy<'f, T> {
    type Target = T;

    fn deref<'l>(&'l self) -> &'l Self::Target {
        self.force();
        match unsafe { &*self.state.get() } {
            &LazyState::Evaluated(ref val) => val,
            _ => unreachable!(),
        }
    }
}

impl<'f, T> DerefMut for Lazy<'f, T> {
    fn deref_mut<'l>(&'l mut self) -> &'l mut Self::Target {
        self.force();
        match unsafe { &mut *self.state.get() } {
            &mut LazyState::Evaluated(ref mut val) => val,
            _ => unreachable!(),
        }
    }
}

struct LazyResultState<'f, T, E>
where
    E: Debug,
{
    inner: Option<Lazy<'f, StdResult<T, E>>>,
    value: Option<T>,
}

impl<'f, T, E: Debug> LazyResultState<'f, T, E> {
    fn new<F: FnOnce() -> StdResult<T, E> + 'f>(f: F) -> Self {
        LazyResultState {
            inner: Some(Lazy::new(f)),
            value: None,
        }
    }

    fn force(&mut self) -> StdResult<(), E> {
        if let Some(lazy) = self.inner.take() {
            assert!(self.value.is_none());
            self.value.replace(lazy.into_inner()?);
        }
        Ok(())
    }

    fn expect_force(&mut self) {
        self.force().expect("Expected lazy evaluation to succeed");
    }

    fn into_inner(mut self) -> T {
        self.expect_force();
        self.value.unwrap()
    }

    fn get(&mut self) -> &T {
        self.expect_force();
        self.value.as_ref().unwrap()
    }

    fn get_mut(&mut self) -> &mut T {
        self.expect_force();
        self.value.as_mut().unwrap()
    }
}

/// `LazyResult` is a simple wrapper around `Lazy` which makes it easier to use
/// when the evaluation result is a Result<T, E>, instead of just a value.
pub struct LazyResult<'f, T, E>
where
    E: Debug,
{
    state: UnsafeCell<LazyResultState<'f, T, E>>,
}

impl<'f, T, E: Debug> LazyResult<'f, T, E> {
    pub fn new<F: FnOnce() -> StdResult<T, E> + 'f>(f: F) -> Self {
        LazyResult {
            state: UnsafeCell::new(LazyResultState::new(f)),
        }
    }

    fn mutable_state(&self) -> &mut LazyResultState<'f, T, E> {
        unsafe { &mut *self.state.get() }
    }

    /// Force evaluation of the wrapped `Lazy`. If this results in an error, it
    /// will be returned directly here. Repeated calls to this function will
    /// just return `Ok(())`.
    ///
    /// Any of this struct's accessors also force evaluation internally, but
    /// they do not return the error: they just `expect` `Ok`. If you want to
    /// handle the error yourself, without panicing, you must call this function
    /// before accessing the inner value.
    ///
    /// If evaluation results in an error, it will only be returned exactly once
    /// (to avoid requiring errors to be `Clone`). After that, calls to any
    /// other function on this structure will panic.
    pub fn force(&self) -> StdResult<(), E> {
        self.mutable_state().force()
    }

    fn expect_force(&self) {
        self.mutable_state().expect_force()
    }

    /// Return the wrapped evaluated value.
    ///
    /// If evaluation results in an `Err`, and you didn't call `force` manually
    /// first, then this function will panic.
    pub fn into_inner(self) -> T {
        self.state.into_inner().into_inner()
    }
}

impl<'f, T, E: Debug> Deref for LazyResult<'f, T, E> {
    type Target = T;

    /// Borrow the wrapped evaluated value.
    ///
    /// If evaluation results in an `Err`, and you didn't call `force` manually
    /// first, then this function will panic.
    fn deref<'l>(&'l self) -> &'l Self::Target {
        self.expect_force();
        self.mutable_state().get()
    }
}

impl<'f, T, E: Debug> DerefMut for LazyResult<'f, T, E> {
    /// Mutably borrow the wrapped evaluated value.
    ///
    /// If evaluation results in an `Err`, and you didn't call `force` manually
    /// first, then this function will panic.
    fn deref_mut<'l>(&'l mut self) -> &'l mut Self::Target {
        self.expect_force();
        self.mutable_state().get_mut()
    }
}
