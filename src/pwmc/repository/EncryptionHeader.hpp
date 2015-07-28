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

#ifndef pwmc_repository_EncryptionHeader_HPP
#define pwmc_repository_EncryptionHeader_HPP

#include <cstddef>
#include <string>
#include <vector>

#include "pwmc/config/Configuration.hpp"

namespace pwm
{
namespace git
{
class Repository;
}

namespace repository
{
std::string getEncryptionHeaderPath(git::Repository const& repository);

class EncryptionHeader
{
public:
	EncryptionHeader(git::Repository const& repository);

	EncryptionHeader(EncryptionHeader const&) = default;
	EncryptionHeader(EncryptionHeader&&) = default;
	EncryptionHeader& operator=(EncryptionHeader const&) = default;
	EncryptionHeader& operator=(EncryptionHeader&&) = default;

	~EncryptionHeader();

	std::vector<uint8_t> getSalt() const;
	std::size_t getKeySize() const;
	int getWorkFactor() const;
	int getParallelizationFactor() const;

private:
	std::string path;
	config::ConfigurationData data;
};
}
}

#endif
