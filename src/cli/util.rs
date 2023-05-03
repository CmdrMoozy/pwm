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

use crate::configuration;
use anyhow::{bail, Result};
use std::path::PathBuf;

pub(crate) fn get_repository_path(repository: Option<PathBuf>) -> Result<PathBuf> {
    Ok(match repository {
        None => match configuration::get()?.default_repository.as_ref() {
            None => bail!("no repository path specified: try passing one as a command-line argument, or setting the 'default_repository' configuration key"),
            Some(r) => r.into(),
        },
        Some(r) => r,
    })
}
