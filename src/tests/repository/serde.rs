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

use ::repository::Repository;
use ::repository::serde::*;
use sodiumoxide::randombytes::randombytes;
use ::tests::tempdir::TempDir;
use ::util::data::SensitiveData;

#[test]
fn test_export_import_round_trip_ascii() {
    let serialized: String;
    let plaintext = "arbitrary test password".to_owned();
    let plaintext_sd = SensitiveData::from(plaintext.as_bytes().to_vec());

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = TempDir::new("pwm-test").unwrap();
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        for path in &paths {
            repository.write_encrypt(&repository.path(path).unwrap(), plaintext_sd.clone())
                .unwrap();
        }
        serialized = export_serialize(&repository).unwrap();
    }

    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("raboof".as_bytes().to_vec())))
        .unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&repository, serialized.as_str()).unwrap();
    for path in &paths {
        assert_eq!(plaintext_sd,
                   repository.read_decrypt(&repository.path(path).unwrap()).unwrap());
    }
}

#[test]
fn test_export_import_round_trip_binary() {
    let serialized: String;
    let plaintext = SensitiveData::from(randombytes(1024));

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = TempDir::new("pwm-test").unwrap();
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        for path in &paths {
            repository.write_encrypt(&repository.path(path).unwrap(), plaintext.clone()).unwrap();
        }
        serialized = export_serialize(&repository).unwrap();
    }

    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("raboof".as_bytes().to_vec())))
        .unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&repository, serialized.as_str()).unwrap();
    for path in &paths {
        assert_eq!(plaintext,
                   repository.read_decrypt(&repository.path(path).unwrap()).unwrap());
    }
}
