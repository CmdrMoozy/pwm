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

use bdrck::flags::value::Values;
use cli;
use configuration;
use error::*;
use std::path::PathBuf;

pub fn get_repository_path(values: &Values) -> Result<PathBuf> {
    Ok(match values.get_single(&cli::REPOSITORY_SPEC.name) {
        None => match configuration::get()?.default_repository.as_ref() {
            None => return Err(Error::InvalidArgument(format_err!("No repository path specified. Try the '{}' command option, or setting the 'default_repository' configuration key.", ::cli::REPOSITORY_SPEC.name))),
            Some(r) => r.into(),
        },
        Some(r) => r.into(),
    })
}
