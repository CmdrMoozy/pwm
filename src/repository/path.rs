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

use crate::error::*;
use std::path::Path as StdPath;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Path {
    relative_path: PathBuf,
    absolute_path: PathBuf,
}

impl Path {
    pub fn new<WorkdirPath: AsRef<StdPath>, RelativePath: AsRef<StdPath>>(
        workdir_path: WorkdirPath,
        relative_path: RelativePath,
    ) -> Result<Path> {
        let mut absolute_path = PathBuf::from(workdir_path.as_ref());
        absolute_path.push(relative_path.as_ref());
        Ok(Path {
            relative_path: PathBuf::from(relative_path.as_ref()),
            absolute_path: absolute_path,
        })
    }

    pub fn relative_path(&self) -> &StdPath {
        self.relative_path.as_path()
    }

    pub fn absolute_path(&self) -> &StdPath {
        self.absolute_path.as_path()
    }

    pub fn to_str(&self) -> Result<&str> {
        match self.relative_path.to_str() {
            Some(s) => Ok(s),
            None => bail!("path contains non-unicode characters"),
        }
    }
}
