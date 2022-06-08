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

#[cfg(test)]
mod configuration;
#[cfg(test)]
mod crypto;
// Tests which verify the on disk format hasn't changed (we can still interpret old repositories,
// and repositories we create use the same format).
#[cfg(test)]
mod odf;
#[cfg(test)]
mod repository;
#[cfg(test)]
mod util;
#[cfg(all(test, feature = "wifiqr"))]
mod wifiqr;

use bdrck::crypto::secret::Secret;

#[cfg(test)]
pub(crate) fn str_secret(s: &str) -> Secret {
    let sb = s.as_bytes();
    let mut ret = Secret::with_len(sb.len()).unwrap();
    unsafe {
        ret.as_mut_slice().copy_from_slice(sb);
    }
    ret
}

#[cfg(test)]
pub(crate) fn random_secret(len: usize) -> Secret {
    let mut s = Secret::with_len(len).unwrap();
    bdrck::crypto::util::randombytes_into_secret(&mut s);
    s
}
