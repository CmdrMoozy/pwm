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

use crate::cli;
use crate::configuration;
use crate::error::*;
use std::path::PathBuf;

pub fn get_repository_path(repository: Option<PathBuf>) -> Result<PathBuf> {
    Ok(match repository {
        None => match configuration::get()?.default_repository.as_ref() {
            None => return Err(Error::InvalidArgument(format!("no repository path specified: try the '{}' command option, or setting the 'default_repository' configuration key", cli::REPOSITORY_SPEC.get_name()))),
            Some(r) => r.into(),
        },
        Some(r) => r,
    })
}
