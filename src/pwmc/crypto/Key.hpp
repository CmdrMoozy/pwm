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

#ifndef pwmc_crypto_Passphrase_HPP
#define pwmc_crypto_Passphrase_HPP

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace pwm
{
namespace crypto
{
constexpr std::size_t DEFAULT_KEY_SIZE_OCTETS = 256 / 8;
constexpr int DEFAULT_SCRYPT_WORK_FACTOR = 20;
constexpr int DEFAULT_SCRYPT_PARALLELIZATION_FACTOR = 1;
constexpr std::size_t DEFAULT_SALT_SIZE = 16;
constexpr std::size_t DEFAULT_IV_SIZE_OCTECTS = 128 / 8;

class Key
{
public:
	Key(std::string const &p, std::vector<uint8_t> const &s,
	    std::size_t ks = DEFAULT_KEY_SIZE_OCTETS,
	    int sw = DEFAULT_SCRYPT_WORK_FACTOR,
	    int sp = DEFAULT_SCRYPT_PARALLELIZATION_FACTOR);
	Key(std::string const &p, std::string const &s,
	    std::size_t ks = DEFAULT_KEY_SIZE_OCTETS,
	    int sw = DEFAULT_SCRYPT_WORK_FACTOR,
	    int sp = DEFAULT_SCRYPT_PARALLELIZATION_FACTOR);
	Key(const std::string &p, std::size_t ks = DEFAULT_KEY_SIZE_OCTETS,
	    int sw = DEFAULT_SCRYPT_WORK_FACTOR,
	    int sp = DEFAULT_SCRYPT_PARALLELIZATION_FACTOR);

	Key(const Key &) = default;
	Key(Key &&) = default;
	Key &operator=(const Key &) = default;
	Key &operator=(Key &&) = default;

	~Key() = default;

	std::vector<uint8_t> const &getSalt() const;
	std::vector<uint8_t> const &getKey() const;

private:
	const std::vector<uint8_t> salt;
	std::vector<uint8_t> key;
};
}
}

#endif
