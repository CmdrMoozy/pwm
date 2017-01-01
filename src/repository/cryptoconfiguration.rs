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

use sodiumoxide::crypto::pwhash::{MemLimit, OpsLimit, Salt, SALTBYTES};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CryptoConfiguration {
    salt: [u8; SALTBYTES],
    mem_limit: usize,
    ops_limit: usize,
}

impl CryptoConfiguration {
    pub fn get_salt(&self) -> Salt { Salt(self.salt.clone()) }

    pub fn get_mem_limit(&self) -> MemLimit { MemLimit(self.mem_limit) }

    pub fn get_ops_limit(&self) -> OpsLimit { OpsLimit(self.ops_limit) }
}
