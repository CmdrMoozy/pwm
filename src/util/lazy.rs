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

use crate::error::{Error, Result};
use once_cell::sync::Lazy;
use std::cell::Cell;
use std::result::Result as StdResult;

pub(crate) struct RepeatableResult<T, E: Into<Error>>(StdResult<T, Cell<Option<E>>>);

impl<T, E: Into<Error>> RepeatableResult<T, E> {
    pub(crate) fn get(&self) -> Result<&T> {
        self.0.as_ref().map_err(|e| e.take().unwrap().into())
    }

    pub(crate) fn get_mut(&mut self) -> Result<&mut T> {
        self.0.as_mut().map_err(|e| e.take().unwrap().into())
    }
}

pub(crate) type LazyResult<T, E> =
    Lazy<RepeatableResult<T, E>, Box<dyn FnOnce() -> RepeatableResult<T, E>>>;

pub(crate) fn new_lazy_result<T, E: Into<Error>, F: FnOnce() -> StdResult<T, E> + 'static>(
    f: F,
) -> LazyResult<T, E> {
    Lazy::new(Box::new(move || {
        RepeatableResult(f().map_err(|e| Cell::new(Some(e))))
    }))
}
