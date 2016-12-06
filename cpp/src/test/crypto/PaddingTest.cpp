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

#include <cstddef>

#include <gcrypt.h>

#include "pwmc/crypto/Padding.hpp"
#include "pwmc/crypto/Util.hpp"

namespace
{
constexpr std::size_t PADDING_TEST_DATA_SIZE_BYTES = 123;
constexpr int PADDING_TEST_ALGORITHM = GCRY_CIPHER_AES256;
constexpr std::size_t PADDING_TEST_ALGORITHM_BLOCK_SIZE_BYTES = 16;
}

TEST_CASE("Test data padding round-trip.", "[Crypto]")
{
	std::vector<uint8_t> original = pwm::crypto::util::generateRandomBytes(
	        PADDING_TEST_DATA_SIZE_BYTES,
	        pwm::crypto::util::RandomQuality::WEAK);
	std::vector<uint8_t> padded{original};
	pwm::crypto::padding::pad(padded, PADDING_TEST_ALGORITHM);
	std::vector<uint8_t> unpadded{padded};
	pwm::crypto::padding::unpad(unpadded);

	CHECK(padded.size() % PADDING_TEST_ALGORITHM_BLOCK_SIZE_BYTES == 0);
	CHECK(unpadded == original);
}
