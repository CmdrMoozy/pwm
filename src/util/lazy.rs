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

struct Factory<'a, T> {
    f: Box<dyn FnOnce() -> T + 'a>,
}

impl<'a, T> Factory<'a, T> {
    fn new<F: 'a + FnOnce() -> T>(f: F) -> Factory<'a, T> {
        Factory { f: Box::new(f) }
    }

    fn build(self) -> T {
        (self.f)()
    }
}

enum LazyState<'a, T> {
    Evaluated(T),
    Evaluating,
    Unevaluated(Factory<'a, T>),
}

pub struct Lazy<'a, T> {
    state: UnsafeCell<LazyState<'a, T>>,
}

impl<'a, T> Lazy<'a, T> {
    pub fn new<F: 'a + FnOnce() -> T>(f: F) -> Lazy<'a, T> {
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
