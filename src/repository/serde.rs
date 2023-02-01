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

use crate::repository::Repository;
use crate::util;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contents {
    pub contents: HashMap<String, String>,
}

pub fn export(repository: &Repository) -> Result<Contents> {
    let mut contents: Contents = Contents {
        contents: HashMap::new(),
    };

    for path in repository.list(None)? {
        let plaintext = util::secret::encode(&repository.read_decrypt(&path)?);
        contents
            .contents
            .insert(path.to_str()?.to_owned(), plaintext);
    }

    Ok(contents)
}

pub fn export_serialize(repository: &Repository) -> Result<String> {
    Ok(to_string_pretty(&export(repository)?)?)
}

pub fn import(repository: &mut Repository, contents: Contents) -> Result<()> {
    for (path, plaintext) in contents.contents {
        let path = repository.path(path)?;
        repository.write_encrypt(&path, util::secret::decode(&plaintext)?, None)?;
    }
    Ok(())
}

pub fn import_deserialize(repository: &mut Repository, s: &str) -> Result<()> {
    import(repository, from_str(s)?)
}
