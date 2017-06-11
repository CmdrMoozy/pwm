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

use error::{Error, Result};
use git2;
use git2::{Commit, ErrorClass, ErrorCode, Index, ObjectType, Oid, Repository, Signature, Tree};
use std::collections::vec_deque::VecDeque;
use std::path::{Path, PathBuf};

static EMPTY_TREE_OID: &'static str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// Open a typical, non-bare Git repository. The given path is used for
/// discovery, so this will work as expected even if the provided path is a
/// subdirectory of the real repository. The repository can optionally be
/// created, at the exact directory specified, if one does not already exist.
pub fn open_repository<P: AsRef<Path>>(path: P, create: bool) -> Result<Repository> {
    let path = path.as_ref();
    match Repository::open(path) {
        Ok(repository) => Ok(repository),
        Err(error) => {
            if create &&
               (error.class() == ErrorClass::Os || error.class() == ErrorClass::Repository) &&
               error.code() == ErrorCode::NotFound {
                Ok(Repository::init(path)?)
            } else {
                Err(Error::from(error))
            }
        },
    }
}

/// Return the given repository's working directory. Since pwm exclusively
/// deals with normal, non-bare repositories, it is considered an error if the
/// given repository does not have a working directory.
pub fn get_repository_workdir(repository: &Repository) -> Result<&Path> {
    match repository.workdir() {
        Some(path) => Ok(path),
        None => bail!("Repository has no workdir"),
    }
}

fn get_signature_or_default(repository: &Repository,
                            signature: Option<&Signature>)
                            -> Result<Signature<'static>> {
    match signature {
        Some(s) => Ok(s.to_owned()),
        None => Ok(repository.signature()?),
    }
}

fn get_head_commit(repository: &Repository) -> Result<Option<Commit>> {
    match repository.head() {
        Ok(r) => {
            let resolved = r.resolve()?;
            let object = resolved.peel(ObjectType::Commit)?;
            Ok(Some(object.into_commit()
                .map_err(|_| git2::Error::from_str("Resolving head commit failed."))?))
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

fn get_head_tree(repository: &Repository) -> Result<Tree> {
    let tree_id = get_head_commit(repository)
        ?
        .map_or(Oid::from_str(EMPTY_TREE_OID).unwrap(), |c| c.tree_id());
    Ok(repository.find_tree(tree_id)?)
}

/// Recursively list all of the contents of the given repository's HEAD tree,
/// returning the listing as a vector of paths.
pub fn get_repository_listing(repository: &Repository, path_filter: &Path) -> Result<Vec<PathBuf>> {
    let mut listing: Vec<PathBuf> = vec![];

    let mut pending_trees: VecDeque<(Tree, PathBuf)> = VecDeque::new();
    pending_trees.push_back((get_head_tree(repository)?, PathBuf::new()));
    while !pending_trees.is_empty() {
        let (tree, prefix) = pending_trees.pop_front().unwrap();

        let mut subtrees: VecDeque<(Tree, PathBuf)> = tree.iter()
            .filter(|entry| entry.kind().unwrap_or(ObjectType::Any) == ObjectType::Tree)
            .map(|entry| {
                let mut path: PathBuf = prefix.clone();
                path.push(entry.name().unwrap());
                (entry.to_object(repository).unwrap().into_tree().ok().unwrap(), path)
            })
            .collect();
        pending_trees.append(&mut subtrees);

        let mut entries: Vec<PathBuf> = tree.iter()
            .filter(|entry| entry.kind().unwrap_or(ObjectType::Any) != ObjectType::Tree)
            .map(|entry| {
                let mut path: PathBuf = prefix.clone();
                path.push(entry.name().unwrap());
                path
            })
            .filter(|entry| entry.starts_with(path_filter))
            .collect();
        listing.append(&mut entries);
    }

    Ok(listing)
}

fn commit_tree(repository: &Repository,
               author: Option<&Signature>,
               committer: Option<&Signature>,
               message: &str,
               tree: Tree)
               -> Result<Oid> {
    let head = get_head_commit(repository)?;
    let parents = match head {
        Some(h) => vec![h],
        None => vec![],
    };

    let parent_refs = parents.iter().collect::<Vec<&Commit>>();
    let parent_tree_id: Oid = parent_refs.get(0)
        .map_or(Oid::from_str(EMPTY_TREE_OID).unwrap(), |p| p.tree_id());

    // If this commit is empty (e.g., its tree is identical to its parent's), don't
    // create a new commit.
    if tree.id() == parent_tree_id {
        return Ok(parent_tree_id);
    }

    let oid = repository.commit(Some("HEAD"),
                &get_signature_or_default(repository, author)?,
                &get_signature_or_default(repository, committer)?,
                message,
                &tree,
                parent_refs.as_slice())?;

    Ok(oid)
}

/// Commit any changes to the files at the given relative paths in the given
/// repository. If no author and comitter Signatures are provided, default
/// Signatures will be used instead from Git's configuration. Empty commits
/// will not be created; if there were no changes to the given paths, the
/// existing HEAD OID will be returned instead.
pub fn commit_paths(repository: &Repository,
                    author: Option<&Signature>,
                    committer: Option<&Signature>,
                    message: &str,
                    paths: &[&Path])
                    -> Result<Oid> {
    let mut index: Index = repository.index()?;

    let workdir: PathBuf = PathBuf::from(get_repository_workdir(repository)?);
    for path in paths {
        let mut absolute_path = workdir.clone();
        absolute_path.push(path);

        if absolute_path.exists() {
            index.add_path(path)?;
        } else {
            index.remove_path(path)?;
        }
    }

    // Commit our changes to the index to disk. This prevents a bug where, e.g.
    // when committing a newly added file, the index will show the newly added file
    // as deleted + untracked.
    index.write()?;
    // Write the index out as a tree so we can commit the tree.
    let oid = index.write_tree()?;
    let tree = repository.find_tree(oid)?;

    commit_tree(repository, author, committer, message, tree)
}
