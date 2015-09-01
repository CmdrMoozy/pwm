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

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <vector>

#include "pwmc/crypto/Util.hpp"
#include "pwmc/util/MemoryFile.hpp"

namespace
{
constexpr std::size_t WRITE_TEST_COUNT = 100;
constexpr std::size_t WRITE_TEST_LENGTH_MIN = 10;
constexpr std::size_t WRITE_TEST_LENGTH_MAX = 2000;
constexpr std::size_t WRITE_TEST_CHUNK_SIZE_MIN = 250;
}

TEST_CASE("Test memory file writing", "[MemoryFile]")
{
	for(std::size_t i = 0; i < WRITE_TEST_COUNT; ++i)
	{
		pwm::util::MemoryFile file;
		std::size_t fileLength =
		        pwm::crypto::util::generateRandomNumber<
		                pwm::crypto::util::RandomQuality::WEAK>(
		                WRITE_TEST_LENGTH_MIN, WRITE_TEST_LENGTH_MAX);
		const std::size_t totalFileLength = fileLength;
		std::vector<uint8_t> contents =
		        pwm::crypto::util::generateRandomBytes(
		                fileLength,
		                pwm::crypto::util::RandomQuality::WEAK);
		std::size_t offset = 0;

		while(fileLength > 0)
		{
			std::size_t chunkLength =
			        std::min(fileLength, WRITE_TEST_CHUNK_SIZE_MIN);
			fileLength -= chunkLength;
			REQUIRE(chunkLength ==
			        file.write(contents.data() + offset,
			                   chunkLength));
			offset += chunkLength;
		}

		file.flush();
		CHECK(totalFileLength == file.size());
		CHECK(std::equal(contents.data(),
		                 contents.data() + contents.size(),
		                 file.data()));
	}
}
