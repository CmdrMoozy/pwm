/*
 * pwm - A simple password manager for Linux.
 * Copyright (C) 2015  Axel Rasmussen
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "Key.hpp"

#include <cstddef>

#include "pwmc/crypto/checkReturn.hpp"
#include "pwmc/crypto/Util.hpp"

namespace
{
constexpr std::size_t KEY_SIZE_OCTETS = 512 / 8;
constexpr int SCRYPT_WORK_FACTOR = 20;
constexpr int SCRYPT_PARALLELIZATION_FACTOR = 1;
constexpr std::size_t DEFAULT_SALT_SIZE = 16;
}

namespace pwm
{
namespace crypto
{
Key::Key(const std::string &p, const std::vector<uint8_t> &s)
        : salt(s), key(KEY_SIZE_OCTETS, 0)
{
	checkReturn(gcry_kdf_derive(p.data(), p.length(), GCRY_KDF_SCRYPT,
	                            SCRYPT_WORK_FACTOR, salt.data(),
	                            salt.size(), SCRYPT_PARALLELIZATION_FACTOR,
	                            KEY_SIZE_OCTETS, key.data()));
}

Key::Key(std::string const &p, std::string const &s)
        : Key(p,
              std::vector<uint8_t>(
                      reinterpret_cast<uint8_t const *>(s.data()),
                      reinterpret_cast<uint8_t const *>(s.data() + s.length())))
{
}

Key::Key(const std::string &p) : Key(p, util::generateSalt(DEFAULT_SALT_SIZE))
{
}

const std::vector<uint8_t> &Key::getSalt() const
{
	return salt;
}

const std::vector<uint8_t> &Key::getKey() const
{
	return key;
}
}
}
