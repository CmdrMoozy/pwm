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

#include "Util.hpp"

#include <stdexcept>

#include <gcrypt.h>

namespace
{
gcry_random_level_t
randomQualityToGcryptLevel(pwm::crypto::util::RandomQuality quality)
{
	switch(quality)
	{
	case pwm::crypto::util::RandomQuality::WEAK:
		return GCRY_WEAK_RANDOM;
	case pwm::crypto::util::RandomQuality::STRONG:
		return GCRY_STRONG_RANDOM;
	case pwm::crypto::util::RandomQuality::VERY_STRONG:
		return GCRY_VERY_STRONG_RANDOM;
	}

	throw std::runtime_error("Unsupported random quality value.");
}
}

namespace pwm
{
namespace crypto
{
namespace util
{
std::vector<uint8_t> generateSalt(std::size_t length)
{
	std::vector<uint8_t> salt(length, 0);
	gcry_randomize(salt.data(), length, GCRY_STRONG_RANDOM);
	return salt;
}

std::vector<uint8_t> generateRandomBytes(std::size_t length,
                                         RandomQuality quality)
{
	std::vector<uint8_t> bytes(length, 0);
	gcry_randomize(bytes.data(), length,
	               randomQualityToGcryptLevel(quality));
	return bytes;
}
}
}
}
