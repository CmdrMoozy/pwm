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
use ::repository::configuration::*;
use ::repository::path::*;
use sodiumoxide::crypto::pwhash;
use sodiumoxide::randombytes::randombytes;
use ::tests::tempdir::TempDir;
use ::util::data::SensitiveData;

#[test]
fn test_wrong_master_password_fails() {
    let repository_dir = TempDir::new("pwm-test").unwrap();

    {
        let _repository = Repository::new(repository_dir.path(),
                                          true,
                                          Some(SensitiveData::from("foobar")))
            .unwrap();
    }

    let repository_result = Repository::new(repository_dir.path(),
                                            false,
                                            Some(SensitiveData::from("barbaz")));
    assert!(repository_result.is_err());
}

#[test]
fn test_crypto_configuration_modification() {
    let repository_dir = TempDir::new("pwm-test").unwrap();
    let initial_config: Option<Configuration>;
    let new_config = Configuration::new(pwhash::gen_salt(),
                                        pwhash::MemLimit(123),
                                        pwhash::OpsLimit(234));

    // Save the default configuration, and change to our new configuration.
    {
        let mut repository = Repository::new(repository_dir.path(),
                                             true,
                                             Some(SensitiveData::from("foobar")))
            .unwrap();
        initial_config = Some(repository.get_crypto_configuration().unwrap());
        repository.set_crypto_configuration(new_config.clone()).unwrap();
    }

    let initial_config = initial_config.unwrap();

    // Assert that the initial configuration and the new configuration are
    // completely different, so we can test that *all* fields are persisted.
    assert_ne!(initial_config.get_salt(), new_config.get_salt());
    assert_ne!(initial_config.get_mem_limit().0,
               new_config.get_mem_limit().0);
    assert_ne!(initial_config.get_ops_limit().0,
               new_config.get_ops_limit().0);

    // Try re-opening the repository, and checking that we get the same
    // configuration we set. Then, reset the configuration.
    {
        let mut repository = Repository::new(repository_dir.path(),
                                             false,
                                             Some(SensitiveData::from("foobar")))
            .unwrap();
        let loaded_config = repository.get_crypto_configuration().unwrap();

        assert_eq!(new_config, loaded_config);

        repository.reset_crypto_configuration().unwrap();
    }

    // Check that the reset is persisted.
    {
        let repository = Repository::new(repository_dir.path(),
                                         false,
                                         Some(SensitiveData::from("foobar")))
            .unwrap();
        let loaded_config = repository.get_crypto_configuration().unwrap();

        // NOTE: In real use, the salt would probably have changed since a new one is
        // generated each time pwm is executed (the "default crypto configuration" is
        // stored in a lazy_static!).
        assert_eq!(initial_config.get_salt(), loaded_config.get_salt());
        assert_eq!(initial_config.get_mem_limit().0,
                   loaded_config.get_mem_limit().0);
        assert_eq!(initial_config.get_ops_limit().0,
                   loaded_config.get_ops_limit().0);
    }
}

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
