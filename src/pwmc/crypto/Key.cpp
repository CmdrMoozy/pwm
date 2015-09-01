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

#include <gcrypt.h>

#include "pwmc/crypto/checkReturn.hpp"
#include "pwmc/crypto/Util.hpp"

namespace pwm
{
namespace crypto
{
Key::Key(const std::string &p, const std::vector<uint8_t> &s, std::size_t ks,
         int sw, int sp)
        : salt(s), key(ks, 0)
{
	checkReturn(gcry_kdf_derive(
	        p.data(), p.length(), GCRY_KDF_SCRYPT, sw, salt.data(),
	        salt.size(), static_cast<unsigned long>(sp), ks, key.data()));
}

Key::Key(std::string const &p, std::string const &s, std::size_t ks, int sw,
         int sp)
        : Key(p,
              std::vector<uint8_t>(
                      reinterpret_cast<uint8_t const *>(s.data()),
                      reinterpret_cast<uint8_t const *>(s.data() + s.length())),
              ks, sw, sp)
{
}

Key::Key(const std::string &p, std::size_t ks, int sw, int sp)
        : Key(p, util::generateSalt(DEFAULT_SALT_SIZE), ks, sw, sp)
{
}

std::vector<uint8_t> const &Key::getSalt() const
{
	return salt;
}

std::vector<uint8_t> const &Key::getKey() const
{
	return key;
}
}
}
