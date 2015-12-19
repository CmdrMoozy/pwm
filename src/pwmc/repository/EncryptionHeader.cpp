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

#include "EncryptionHeader.hpp"

#include <cassert>
#include <map>

#include <bdrck/fs/Util.hpp>

#include "pwmc/config/deserializeConfiguration.hpp"
#include "pwmc/config/Key.hpp"
#include "pwmc/config/serializeConfiguration.hpp"
#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/Util.hpp"
#include "pwmc/git/Repository.hpp"
#include "pwmc/util/Base64.hpp"

namespace
{
const pwm::config::Key HEADER_KEY_SALT("salt");
const pwm::config::Key HEADER_KEY_KEY_SIZE("keysize");
const pwm::config::Key HEADER_KEY_WORK_FACTOR("workfactor");
const pwm::config::Key
        HEADER_KEY_PARALLELIZATION_FACTOR("parallelizationfactor");

const std::vector<uint8_t> DEFAULT_SALT =
        pwm::crypto::util::generateSalt(pwm::crypto::DEFAULT_SALT_SIZE);

const std::map<pwm::config::Key, std::string> DEFAULT_HEADER_VALUES{
        {HEADER_KEY_SALT, ""},
        {HEADER_KEY_KEY_SIZE,
         std::to_string(pwm::crypto::DEFAULT_KEY_SIZE_OCTETS)},
        {HEADER_KEY_WORK_FACTOR,
         std::to_string(pwm::crypto::DEFAULT_SCRYPT_WORK_FACTOR)},
        {HEADER_KEY_PARALLELIZATION_FACTOR,
         std::to_string(pwm::crypto::DEFAULT_SCRYPT_PARALLELIZATION_FACTOR)}};
}

namespace pwm
{
namespace repository
{
std::string getEncryptionHeaderPath(git::Repository const &repository)
{
	static const std::string ENCRYPTION_HEADER_FILE = ".header";
	return bdrck::fs::combinePaths(repository.getWorkDirectoryPath(),
	                               ENCRYPTION_HEADER_FILE);
}

EncryptionHeader::EncryptionHeader(git::Repository const &repository)
        : path(getEncryptionHeaderPath(repository)),
          data(config::deserializeConfiguration(path))
{
	data.apply(config::ConfigurationData(DEFAULT_HEADER_VALUES), false);

	// If there is no existing salt, generate one. This must be done here
	// rather than in the constant default map at the top, because static
	// variable initialization order is undefined (and encodeBase64
	// depends on a static).
	if(getSalt().size() == 0)
	{
		data.data[HEADER_KEY_SALT] = pwm::util::encodeBase64(
		        DEFAULT_SALT.data(), DEFAULT_SALT.size());
	}
}

EncryptionHeader::~EncryptionHeader()
{
	try
	{
		serializeConfiguration(path, data);
	}
	catch(...)
	{
	}
}

std::vector<uint8_t> EncryptionHeader::getSalt() const
{
	auto it = data.data.find(HEADER_KEY_SALT);
	assert(it != data.data.end());
	return util::decodeBase64(it->second);
}

std::size_t EncryptionHeader::getKeySize() const
{
	auto it = data.data.find(HEADER_KEY_KEY_SIZE);
	assert(it != data.data.end());
	return static_cast<std::size_t>(std::stoull(it->second));
}

int EncryptionHeader::getWorkFactor() const
{
	auto it = data.data.find(HEADER_KEY_WORK_FACTOR);
	assert(it != data.data.end());
	return std::stoi(it->second);
}

int EncryptionHeader::getParallelizationFactor() const
{
	auto it = data.data.find(HEADER_KEY_PARALLELIZATION_FACTOR);
	assert(it != data.data.end());
	return std::stoi(it->second);
}
}
}
