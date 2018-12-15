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

use crate::cli::util;
use crate::error::*;
use crate::piv::{self, impls};
use bdrck::flags::Values;

pub(crate) fn setuppiv(values: Values) -> Result<()> {
    impls::setuppiv(
        &util::get_repository_path(&values)?,
        values.get_required_parsed(&piv::SLOT_SPEC.name)?,
        values.get_required_parsed(&piv::ALGORITHM_SPEC.name)?,
        values.get_required_parsed(&piv::PIN_POLICY_SPEC.name)?,
        values.get_required_parsed(&piv::TOUCH_POLICY_SPEC.name)?,
        values.get_required(&piv::PUBLIC_KEY_SPEC.name),
    )
}

pub(crate) fn addpiv(values: Values) -> Result<()> {
    impls::addpiv(
        &util::get_repository_path(&values)?,
        values.get_required_parsed(&piv::SLOT_SPEC.name)?,
        values.get_required(&piv::PUBLIC_KEY_SPEC.name),
    )
}
