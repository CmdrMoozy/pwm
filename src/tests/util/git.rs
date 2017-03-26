use git2::{Oid, Repository, Signature};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tests::tempdir::TempDir;
use ::util::git::*;

fn get_test_signature() -> Signature<'static> { Signature::now("test", "test@test.com").unwrap() }

#[test]
fn test_open_repository() {
    let tmp_dir = TempDir::new("pwm-tests").unwrap();
    assert!(tmp_dir.path().exists());
    assert!(open_repository(tmp_dir.path(), false).is_err());
    let repository = open_repository(tmp_dir.path(), true).unwrap();
    assert_eq!(tmp_dir.path(), repository.workdir().unwrap());
}

fn write_and_commit(relative_path: &str, contents: &str, repository: &Repository) -> Oid {
    use std::io::Write;

    let relative_path = PathBuf::from(relative_path);
    let mut path = PathBuf::from(repository.workdir().unwrap());
    path.push(relative_path.as_path());

    fs::create_dir_all(path.as_path().parent().unwrap()).unwrap();
    let mut file = File::create(path.as_path()).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    file.flush().unwrap();

    commit_paths(repository,
                 Some(&get_test_signature()),
                 Some(&get_test_signature()),
                 "test commit",
                 &[relative_path.as_path()])
        .unwrap()
}

#[test]
fn test_commit_paths_and_listing() {
    let tmp_dir = TempDir::new("pwm-tests").unwrap();
    assert!(tmp_dir.path().exists());
    let repository = open_repository(tmp_dir.path(), true).unwrap();

    write_and_commit("foo.txt", "test file", &repository);
    write_and_commit("a/b/bar.txt", "another test file", &repository);
    write_and_commit("baz.txt", "yet another test file", &repository);

    let path_filter = PathBuf::new();
    let listing = get_repository_listing(&repository, path_filter.as_path()).unwrap();
    assert_eq!(vec![PathBuf::from("baz.txt"),
                    PathBuf::from("foo.txt"),
                    PathBuf::from("a/b/bar.txt")],
               listing);

}
