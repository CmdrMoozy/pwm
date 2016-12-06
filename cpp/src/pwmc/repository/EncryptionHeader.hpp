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
#include <memory>
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
constexpr char const *ENCRYPTION_HEADER_RELATIVE_PATH = ".header";

class EncryptionHeader
{
public:
	EncryptionHeader(std::shared_ptr<bdrck::git::Repository> r);

	EncryptionHeader(EncryptionHeader const &) = delete;
	EncryptionHeader(EncryptionHeader &&) = default;
	EncryptionHeader &operator=(EncryptionHeader const &) = delete;
	EncryptionHeader &operator=(EncryptionHeader &&) = default;

	~EncryptionHeader();

	std::string const &getPath() const;

	std::vector<uint8_t> getSalt() const;
	std::size_t getKeySize() const;
	int getWorkFactor() const;
	int getParallelizationFactor() const;

private:
	std::shared_ptr<bdrck::git::Repository> repository;
	std::string path;
	std::unique_ptr<bdrck::config::ConfigurationInstance<
	        pwm::proto::EncryptionHeader>>
	        instanceHandle;
	bdrck::config::Configuration<pwm::proto::EncryptionHeader> &instance;
};
}
}

#endif
