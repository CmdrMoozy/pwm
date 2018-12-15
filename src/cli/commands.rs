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

use crate::cli::{self, impls, util};
use crate::error::*;
use bdrck::flags::value::Values;
use std::path::PathBuf;

pub(crate) fn config(values: Values) -> Result<()> {
    impls::config(
        values.get_single(&cli::CONFIG_KEY_SPEC.name),
        values.get_single(&cli::CONFIG_SET_SPEC.name),
    )
}

pub(crate) fn init(values: Values) -> Result<()> {
    impls::init(&util::get_repository_path(&values)?)
}

pub(crate) fn addkey(values: Values) -> Result<()> {
    impls::addkey(&util::get_repository_path(&values)?)
}

pub(crate) fn rmkey(values: Values) -> Result<()> {
    impls::rmkey(&util::get_repository_path(&values)?)
}

pub(crate) fn ls(values: Values) -> Result<()> {
    impls::ls(
        &util::get_repository_path(&values)?,
        values.get_positional_single(&cli::PATH_PREFIX_SPEC.name),
    )
}

pub(crate) fn get(values: Values) -> Result<()> {
    impls::get(
        &util::get_repository_path(&values)?,
        values.get_boolean(&cli::GET_BINARY_SPEC.name),
        if cfg!(feature = "clipboard") {
            values.get_boolean(&cli::GET_CLIPBOARD_SPEC.name)
        } else {
            false
        },
        values.get_positional_single(&cli::PATH_SPEC.name),
    )
}

pub(crate) fn set(values: Values) -> Result<()> {
    impls::set(
        &util::get_repository_path(&values)?,
        values
            .get_single(&cli::SET_KEY_FILE_SPEC.name)
            .map(|p| PathBuf::from(p))
            .as_ref(),
        values.get_boolean(&cli::SET_MULTILINE_SPEC.name),
        values.get_positional_single(&cli::PATH_SPEC.name),
    )
}

pub(crate) fn rm(values: Values) -> Result<()> {
    impls::rm(
        &util::get_repository_path(&values)?,
        values.get_positional_single(&cli::PATH_SPEC.name),
    )
}

pub(crate) fn generate(values: Values) -> Result<()> {
    impls::generate(
        values.get_required_parsed(&cli::GENERATE_PASSWORD_LENGTH_SPEC.name)?,
        values.get_boolean(&cli::GENERATE_EXCLUDE_LETTERS_SPEC.name),
        values.get_boolean(&cli::GENERATE_EXCLUDE_NUMBERS_SPEC.name),
        values.get_boolean(&cli::GENERATE_INCLUDE_SYMBOLS_SPEC.name),
        values.get_single(&cli::GENERATE_CUSTOM_EXCLUDE_SPEC.name),
    )
}

pub(crate) fn export(values: Values) -> Result<()> {
    impls::export(&util::get_repository_path(&values)?)
}

pub(crate) fn import(values: Values) -> Result<()> {
    impls::import(
        &util::get_repository_path(&values)?,
        &values.get_required_as::<PathBuf>(&cli::IMPORT_INPUT_SPEC.name),
    )
}
