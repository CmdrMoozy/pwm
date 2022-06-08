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

use crate::crypto::configuration::*;
use bdrck::testing::temp;
use std::fs;

// Leave commented out, as this will be used if the configuration format changes
// and the testdata needs to be regenerated.
/*
fn write_configuration() {
    use bdrck::crypto::key::Digest;

    let mut c = Configuration::default();
    c.mut_piv_keys().push(crate::piv::util::PivKeyAssociation {
        reader: "PIV device".to_owned(),
        serial: 1234567,
        wrapping_key_digest: Digest::from_bytes(b"Hello, world!"),
        slot: yubirs::piv::id::Key::KeyManagement,
        public_key_pem: b"-----BEGIN PUBLIC KEY----- \
            MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtnonVb4e2SuzTQqHu19y \
            BRb3z3UawUC0QIBTZqbOOMjghpim/eITks2BHPWDxEqmZjxbVjXRLPbXX7D/hkp4 \
            rNtQ9SXic3T3ocZKuqy5ZpNIIgtgWF8PPp74rNaEo25E12Iylt0EKBpEZbise7hP \
            PyvFIfgbloUw5rEcgyBPgUvKksx79MVqMRH2lGbXoMerSAD0S7FC+QHoTbh7I47v \
            T/wLN3lvfNvclKPVxnjVRMGWOoo0koReBHO/+FawA2Lk2NuKEuIVLpVTs5FErmQc \
            Q5nWPmdeNjQ8JZ8PvHKpQpNVmoko58Gi/UaA2A17BXIUB4uCB7070ey63W1F7Ed6 \
            8QIDAQAB \
            i-----END PUBLIC KEY-----"
            .to_vec(),
    });

    let d = temp::Dir::new("pwm-test").unwrap();
    let ci = ConfigurationInstance::new(d.path().join("config")).unwrap();
    ci.set(c);
    ci.close().unwrap();
    let data = fs::read(d.path().join("config")).unwrap();
    println!("Serialized version is {} bytes", data.len());
}
*/

const CONFIG_WITH_PIV_KEYS: &'static [u8] = include_bytes!("testdata/config_with_piv_keys");
const CONFIG_WITHOUT_PIV_KEYS: &'static [u8] = include_bytes!("testdata/config_without_piv_keys");

#[test]
fn test_deserialize_without_piv_keys() {
    crate::init().unwrap();

    // We should always be able to deserialize a structure without a `piv_keys`
    // entry, regardless of what features are enabled.

    let f = temp::File::new_file().unwrap();
    fs::write(f.path(), CONFIG_WITHOUT_PIV_KEYS).unwrap();

    let ci = ConfigurationInstance::new(f.path()).unwrap();
    let c = ci.get();

    // Just do a very basic check of the data, no need to be super exhaustive.
    assert!(c.get_mem_limit() > 0);
    assert!(c.get_ops_limit() > 0);
}

#[cfg(feature = "piv")]
#[test]
fn test_deserialize_with_piv_keys_works() {
    crate::init().unwrap();

    // If PIV support is enabled, deserializing a structure with a `piv_keys`
    // substructure should succeed.

    let f = temp::File::new_file().unwrap();
    fs::write(f.path(), CONFIG_WITH_PIV_KEYS).unwrap();

    let ci = ConfigurationInstance::new(f.path()).unwrap();
    let c = ci.get();

    assert_eq!(1, c.get_piv_keys().len());
    assert_eq!("PIV device", c.get_piv_keys()[0].reader);
}

#[cfg(not(feature = "piv"))]
#[test]
#[should_panic(expected = "PIV feature is disabled; refusing to load PIV configuration")]
fn test_deserialize_with_piv_keys_panics() {
    crate::init().unwrap();

    // If we were built without PIV support, and we encounter a `piv_keys`
    // structure, we should panic. This is so we don't e.g. skip deserializing
    // it, and then omit the data when we serialize the structure back out.
    // TODO: Be smarter about this in the future (preserve + ignore it).

    let f = temp::File::new_file().unwrap();
    fs::write(f.path(), CONFIG_WITH_PIV_KEYS).unwrap();

    let _ci = ConfigurationInstance::new(f.path()).unwrap();
}
