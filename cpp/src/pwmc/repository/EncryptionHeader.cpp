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

#include <bdrck/git/Commit.hpp>
#include <bdrck/git/Index.hpp>
#include <bdrck/git/Repository.hpp>
#include <bdrck/git/StrArray.hpp>
#include <bdrck/util/Base64.hpp>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/Util.hpp"

namespace
{
constexpr char const *HEADER_CHANGE_MESSAGE =
        "Update encryption header contents.";

std::string getEncryptionHeaderPath(bdrck::git::Repository const &repository)
{
	static const std::string ENCRYPTION_HEADER_FILE{
	        pwm::repository::ENCRYPTION_HEADER_RELATIVE_PATH};
	return bdrck::fs::combinePaths(repository.getWorkDirectoryPath(),
	                               ENCRYPTION_HEADER_FILE);
}

pwm::proto::EncryptionHeader getDefaultEncryptionHeader()
{
	auto defaultSalt =
	        pwm::crypto::util::generateSalt(pwm::crypto::DEFAULT_SALT_SIZE);

	pwm::proto::EncryptionHeader header;
	header.set_key_salt(bdrck::util::encodeBase64(defaultSalt.data(),
	                                              defaultSalt.size()));
	header.set_key_size_octets(pwm::crypto::DEFAULT_KEY_SIZE_OCTETS);
	header.set_key_work_factor(pwm::crypto::DEFAULT_SCRYPT_WORK_FACTOR);
	header.set_key_parallelization_factor(
	        pwm::crypto::DEFAULT_SCRYPT_PARALLELIZATION_FACTOR);
	return header;
}

bdrck::config::ConfigurationIdentifier
getConfigurationIdentifier(std::string const &p)
{
	return {"pwm", p};
}
}

namespace pwm
{
namespace repository
{
EncryptionHeader::EncryptionHeader(std::shared_ptr<bdrck::git::Repository> r)
        : repository(r),
          path(getEncryptionHeaderPath(*repository)),
          instanceHandle(std::make_unique<bdrck::config::ConfigurationInstance<
                                 pwm::proto::EncryptionHeader>>(
                  getConfigurationIdentifier(path),
                  getDefaultEncryptionHeader(), path)),
          instance(bdrck::config::Configuration<pwm::proto::EncryptionHeader>::
                           instance(getConfigurationIdentifier(path)))
{
}

EncryptionHeader::~EncryptionHeader()
{
	// Reset the header configuration instance, to write its contents.
	instanceHandle.reset();
	// NOTE: From this point forward, instance is an invalid reference.

	// Create a Git commit with encryption header changes, if any.
	bdrck::git::Index index(*repository);
	index.addAll({ENCRYPTION_HEADER_RELATIVE_PATH});
	bdrck::git::commitIndex(*repository, HEADER_CHANGE_MESSAGE);
}

std::string const &EncryptionHeader::getPath() const
{
	return path;
}

std::vector<uint8_t> EncryptionHeader::getSalt() const
{
	return bdrck::util::decodeBase64(instance.get().key_salt());
}

std::size_t EncryptionHeader::getKeySize() const
{
	return instance.get().key_size_octets();
}

int EncryptionHeader::getWorkFactor() const
{
	return instance.get().key_work_factor();
}

int EncryptionHeader::getParallelizationFactor() const
{
	return instance.get().key_parallelization_factor();
}
}
}
