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
use std::ops::{Deref, DerefMut};
use std::ptr;

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
