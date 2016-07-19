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

#include "Padding.hpp"

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <stdexcept>

#include <gcrypt.h>

#include "pwmc/crypto/Util.hpp"

namespace pwm
{
namespace crypto
{
namespace padding
{
void pad(std::vector<uint8_t> &plaintext, int algorithm)
{
	std::size_t blockSize = gcry_cipher_get_algo_blklen(algorithm);
	if(blockSize == 0)
	{
		throw std::runtime_error(
		        "Failed to determine algorithm block size.");
	}

	// Prepend the actual size of the plaintext to the plaintext.
	uint64_t size = plaintext.size();
	plaintext.insert(
	        plaintext.begin(), reinterpret_cast<uint8_t const *>(&size),
	        reinterpret_cast<uint8_t const *>(&size) + sizeof(size));

	// Compute the final padded size (a multiple of the block size).
	std::size_t paddedSize = plaintext.size();
	std::size_t blocks = paddedSize / blockSize;
	blocks = paddedSize % blockSize == 0 ? blocks : blocks + 1;
	paddedSize = blocks * blockSize;

	// Fill the padding bytes with random data.
	std::vector<uint8_t> padding = pwm::crypto::util::generateRandomBytes(
	        paddedSize - plaintext.size(),
	        pwm::crypto::util::RandomQuality::STRONG);
	plaintext.insert(plaintext.end(), padding.begin(), padding.end());
	assert(plaintext.size() % blockSize == 0);
}

void unpad(std::vector<uint8_t> &plaintext)
{
	uint64_t realSize;
	std::copy(plaintext.data(), plaintext.data() + sizeof(realSize),
	          reinterpret_cast<uint8_t *>(&realSize));
	plaintext.erase(plaintext.begin(),
	                plaintext.begin() + sizeof(realSize));
	plaintext.resize(realSize);
}
}
}
}
