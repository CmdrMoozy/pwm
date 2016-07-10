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

#include <bdrck/config/Configuration.hpp>

#include "EncryptionHeader.pb.h"

namespace bdrck
{
namespace git
{
class Repository;
}
}

namespace pwm
{
namespace repository
{
std::string getEncryptionHeaderPath(bdrck::git::Repository const &repository);

class EncryptionHeader
{
public:
	EncryptionHeader(bdrck::git::Repository const &repository);

	EncryptionHeader(EncryptionHeader const &) = delete;
	EncryptionHeader(EncryptionHeader &&) = default;
	EncryptionHeader &operator=(EncryptionHeader const &) = delete;
	EncryptionHeader &operator=(EncryptionHeader &&) = default;

	~EncryptionHeader() = default;

	std::vector<uint8_t> getSalt() const;
	std::size_t getKeySize() const;
	int getWorkFactor() const;
	int getParallelizationFactor() const;

private:
	std::string path;
	bdrck::config::ConfigurationInstance<pwm::proto::EncryptionHeader>
	        instanceHandle;
	bdrck::config::Configuration<pwm::proto::EncryptionHeader> &instance;
};
}
}

#endif
