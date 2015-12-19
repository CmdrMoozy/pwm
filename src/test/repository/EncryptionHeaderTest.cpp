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

#include <catch/catch.hpp>

#include <bdrck/fs/TemporaryStorage.hpp>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/git/Repository.hpp"
#include "pwmc/repository/EncryptionHeader.hpp"

TEST_CASE("Test that encryption header default values are populated",
          "[Repository]")
{
	bdrck::fs::TemporaryStorage directory(
	        bdrck::fs::TemporaryStorageType::DIRECTORY);
	pwm::git::Repository repository(directory.getPath());

	// Construct an encryption header, and then destruct it to write the
	// values to the repository.
	{
		pwm::repository::EncryptionHeader header(repository);
	}

	// Construct a new encryption header object, and verify that it
	// contains the correct values.
	pwm::repository::EncryptionHeader header(repository);
	CHECK(pwm::crypto::DEFAULT_SALT_SIZE == header.getSalt().size());
	CHECK(pwm::crypto::DEFAULT_KEY_SIZE_OCTETS == header.getKeySize());
	CHECK(pwm::crypto::DEFAULT_SCRYPT_WORK_FACTOR ==
	      header.getWorkFactor());
	CHECK(pwm::crypto::DEFAULT_SCRYPT_PARALLELIZATION_FACTOR ==
	      header.getParallelizationFactor());
}
