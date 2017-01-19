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

use ::repository::*;
use sodiumoxide::randombytes::randombytes;
use ::tests::tempdir::TempDir;
use ::util::data::SensitiveData;

#[test]
fn test_write_read_round_trip() {
    let repository_dir = TempDir::new("pwm-test").unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));

    {
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar")))
            .unwrap();
        let path = Path::from_repository(&repository, "test").unwrap();
        repository.write_encrypt(&path, plaintext.clone()).unwrap();
    }

    {
        let repository = Repository::new(repository_dir.path(),
                                         false,
                                         Some(SensitiveData::from("foobar")))
            .unwrap();
        let path = Path::from_repository(&repository, "test").unwrap();
        let output_plaintext = repository.read_decrypt(&path).unwrap();
        assert_eq!(&plaintext[..], &output_plaintext[..]);
    }
}

#[test]
fn test_repository_listing() {
    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("foobar")))
        .unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));

    repository.write_encrypt(&Path::from_repository(&repository, "foo/1").unwrap(),
                       plaintext.clone())
        .unwrap();
    repository.write_encrypt(&Path::from_repository(&repository, "bar/2").unwrap(),
                       plaintext.clone())
        .unwrap();
    repository.write_encrypt(&Path::from_repository(&repository, "3").unwrap(),
                       plaintext.clone())
        .unwrap();
    repository.write_encrypt(&Path::from_repository(&repository, "foo/bar/4").unwrap(),
                       plaintext.clone())
        .unwrap();

    let listing: Vec<String> = repository.list(&Path::from_repository(&repository, "").unwrap())
        .unwrap()
        .iter()
        .map(|p| p.to_str().unwrap().to_owned())
        .collect();

    assert_eq!(vec!["3".to_owned(),
                    "bar/2".to_owned(),
                    "foo/1".to_owned(),
                    "foo/bar/4".to_owned()],
               listing);
}
