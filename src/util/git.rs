// pwm - A simple password manager for Linux.
// Copyright (C) 2015  Axel Rasmussen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use ::error::{Error, Result};
use git2::{Commit, ErrorClass, ErrorCode, Index, Oid, Repository, ResetType, Signature, Tree};
use std::path::Path;

pub fn open_repository<P: AsRef<Path>>(path: P, create: bool) -> Result<Repository> {
    let path = path.as_ref();
    match Repository::open(path) {
        Ok(repository) => Ok(repository),
        Err(error) => {
            match create && error.class() == ErrorClass::Os && error.code() == ErrorCode::NotFound {
                false => Err(Error::from(error)),
                true => Ok(try!(Repository::init(path))),
            }
        },
    }
}

fn get_signature_or_default(repository: &Repository,
                            signature: Option<&Signature>)
                            -> Result<Signature<'static>> {
    match signature {
        Some(s) => Ok(s.to_owned()),
        None => Ok(try!(repository.signature())),
    }
}

fn get_head_commit(repository: &Repository) -> Result<Option<Commit>> {
    match repository.head() {
        Ok(r) => {
            match r.target() {
                Some(oid) => Ok(Some(try!(repository.find_commit(oid)))),
                None => Ok(None),
            }
        },
        Err(e) => {
            if e.class() == ErrorClass::Reference && e.code() == ErrorCode::UnbornBranch {
                Ok(None)
            } else {
                Err(Error::from(e))
            }
        },
    }
}

pub fn commit_tree(repository: &Repository,
                   author: Option<&Signature>,
                   committer: Option<&Signature>,
                   message: &str,
                   tree: &Tree)
                   -> Result<Oid> {
    let head = try!(get_head_commit(repository));
    let parents = match head {
        Some(h) => vec![h],
        None => vec![],
    };
    let parent_refs = parents.iter().collect::<Vec<&Commit>>();

    let oid = try!(repository.commit(Some("HEAD"),
                                     &try!(get_signature_or_default(repository, author)),
                                     &try!(get_signature_or_default(repository, committer)),
                                     message,
                                     tree,
                                     parent_refs.as_slice()));

    try!(repository.reset(&try!(repository.find_object(oid.clone(), None)),
                          ResetType::Hard,
                          None));

    Ok(oid)
}

pub fn commit_index(repository: &Repository,
                    author: Option<&Signature>,
                    committer: Option<&Signature>,
                    message: &str)
                    -> Result<Oid> {
    let mut index: Index = try!(repository.index());
    let tree = try!(repository.find_tree(try!(index.write_tree())));
    commit_tree(repository, author, committer, message, &tree)
}

pub fn commit_paths(repository: &Repository,
                    author: Option<&Signature>,
                    committer: Option<&Signature>,
                    message: &str,
                    paths: &[&Path])
                    -> Result<Oid> {
    let mut index: Index = try!(repository.index());
    for path in paths {
        try!(index.add_path(path));
    }
    commit_index(repository, author, committer, message)
}
