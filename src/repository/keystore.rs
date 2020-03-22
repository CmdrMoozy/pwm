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

use crate::crypto::configuration::Configuration;
use crate::error::*;
use crate::util::data::Secret;
use bdrck::crypto::key::AbstractKey;
use bdrck::crypto::keystore::DiskKeyStore;
use failure::format_err;
use std::path::Path;

static MASTER_PASSWORD_PROMPT: &'static str = "Master password: ";
static ADD_KEY_PROMPT: &'static str = "Master password to add: ";
static REMOVE_KEY_PROMPT: &'static str = "Master password to remove: ";

#[cfg(feature = "piv")]
fn find_piv_master_key(crypto_config: &Configuration) -> Result<Option<Box<dyn AbstractKey>>> {
    crate::piv::util::find_master_key(crypto_config)
}

#[cfg(not(feature = "piv"))]
fn find_piv_master_key(_: &Configuration) -> Result<Option<Box<dyn AbstractKey>>> {
    Ok(None)
}

fn open(
    keystore: &mut DiskKeyStore,
    crypto_config: &Configuration,
    password: Option<Secret>,
) -> Result<()> {
    if keystore.is_open() {
        return Ok(());
    }

    if let Some(piv_key) = find_piv_master_key(crypto_config)? {
        if let Err(e) = keystore.open(&piv_key) {
            eprintln!("Failed to use master PIV key ({})", e);
        }
    }

    while !keystore.is_open() {
        let key = crypto_config.get_password_key(
            password.clone(),
            MASTER_PASSWORD_PROMPT,
            /*confirm=*/ false,
        )?;
        if let Err(e) = keystore.open(&key) {
            eprintln!("Invalid master key ({}), try again.", e);
        }

        // Only try once, if a hard-coded password was provided.
        if password.is_some() {
            break;
        }
    }
    Ok(())
}

pub(crate) fn get_keystore<P: AsRef<Path>>(
    path: P,
    allow_create: bool,
    crypto_config: &Configuration,
    password: Option<Secret>,
) -> Result<DiskKeyStore> {
    let mut keystore = DiskKeyStore::new(path.as_ref(), /*force_overwrite=*/ false)?;

    // Check for the case where we really expected an existing key store.
    if !allow_create && !keystore.is_persistable() {
        return Err(Error::NotFound(format_err!(
            "No key store found at '{}'",
            path.as_ref().display()
        )));
    }

    // If this is a newly initialized key store, add an initial wrapping key.
    if !keystore.is_persistable() {
        let key = crypto_config.get_password_key(
            password.clone(),
            ADD_KEY_PROMPT,
            /*confirm=*/ true,
        )?;
        keystore.add_key(&key)?;
    }

    // If this key store needs to be opened, find an appropriate key and do so.
    open(&mut keystore, crypto_config, password)?;

    // Return the fully initialized key store.
    Ok(keystore)
}

pub(crate) fn add_key<K: AbstractKey>(keystore: &mut DiskKeyStore, key: &K) -> Result<()> {
    let was_added = keystore.add_key(key)?;
    if !was_added {
        return Err(Error::InvalidArgument(format_err!(
            "The specified key is already in use, so it was not re-added"
        )));
    }
    Ok(())
}

pub(crate) fn add_password_key(
    crypto_config: &Configuration,
    keystore: &mut DiskKeyStore,
    password: Option<Secret>,
) -> Result<()> {
    add_key(
        keystore,
        &crypto_config.get_password_key(password, ADD_KEY_PROMPT, /*confirm=*/ true)?,
    )
}

fn remove_key<K: AbstractKey>(keystore: &mut DiskKeyStore, key: &K) -> Result<()> {
    let was_removed = keystore.remove_key(key)?;
    if !was_removed {
        return Err(Error::NotFound(format_err!(
            "The specified key is not registered with this repository"
        )));
    }
    Ok(())
}

pub(crate) fn remove_password_key(
    crypto_config: &Configuration,
    keystore: &mut DiskKeyStore,
    password: Option<Secret>,
) -> Result<()> {
    remove_key(
        keystore,
        &crypto_config.get_password_key(password, REMOVE_KEY_PROMPT, /*confirm=*/ false)?,
    )
}
