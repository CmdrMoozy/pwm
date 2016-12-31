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

use ::crypto::padding::*;
use sodiumoxide::randombytes::randombytes;

#[test]
fn test_padding_round_trip() {
    let mut data = randombytes(123);
    let original_data = data.clone();
    pad(&mut data);
    assert!(data.len() > original_data.len());
    unpad(&mut data).unwrap();
    assert_eq!(original_data, data);
}

#[test]
fn test_unpadding_invalid_size() {
    let mut data: Vec<u8> = vec![];
    assert!(unpad(&mut data).is_err());
}
