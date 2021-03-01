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

use crate::crypto::rng::Generator;
use rand::{Rng, RngCore};
use std::collections::HashSet;

#[test]
fn test_rng_output() {
    // Test that we get at least a few distinct values from the RNG if we generate
    // many values. This is kind of a silly test, but it should at least protect
    // against mis-implementing Generator so that it returns the same value every
    // time.
    let mut generator = Generator;
    let values: HashSet<u64> = (0..1000).map(|_| generator.next_u64()).collect();
    assert!(values.len() > 1);
}

#[test]
fn test_rng_range() {
    let mut generator = Generator;
    let min: u64 = 123;
    let max: u64 = 9876;
    for _ in 0..100 {
        let n: u64 = generator.gen_range(min..max);
        assert!(min <= n);
        assert!(n < max);
    }
}
