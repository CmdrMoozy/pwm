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

use crate::util::lazy::Lazy;
use std::sync::{Arc, Mutex};

#[test]
fn test_simple_evaluation() {
    crate::init().unwrap();

    let v = Lazy::new(|| 42);
    assert_eq!(42, *v);
}

#[test]
fn test_single_evaluation() {
    crate::init().unwrap();

    let c = Arc::new(Mutex::new(0));
    let counter = c.clone();

    let v = Lazy::new(|| {
        let mut counter = c.lock().unwrap();
        *counter += 1;
    });

    *v;
    *v;
    assert_eq!(1, *counter.lock().unwrap());
}

#[test]
fn test_no_evaluation() {
    crate::init().unwrap();

    let c = Arc::new(Mutex::new(0));
    let counter = c.clone();

    let _v = Lazy::new(|| {
        let mut counter = c.lock().unwrap();
        *counter += 1;
    });

    assert_eq!(0, *counter.lock().unwrap());
}

#[test]
fn test_into_inner() {
    crate::init().unwrap();

    let v = Lazy::new(|| 42);
    assert_eq!(42, v.into_inner());
}
