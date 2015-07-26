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

#include <cstdint>
#include <string>
#include <vector>

namespace pwm
{
namespace crypto
{
class Key
{
public:
	Key(std::string const&p, std::vector<uint8_t> const& s);
	Key(std::string const& p, std::string const& s);
	explicit Key(const std::string &p);

	Key(const Key &) = default;
	~Key() = default;
	Key &operator=(const Key &) = default;

	const std::vector<uint8_t>& getSalt() const;
	const std::vector<uint8_t>& getKey() const;

private:
	const std::vector<uint8_t> salt;
	std::vector<uint8_t> key;
};
}
}

#endif
