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

#ifndef pwmc_crypto_Util_HPP
#define pwmc_crypto_Util_HPP

#include <cstddef>
#include <cstdint>
#include <limits>
#include <random>
#include <vector>

namespace pwm
{
namespace crypto
{
namespace util
{
enum class RandomQuality
{
	WEAK,
	STRONG,     // Use for session keys and similar purposes
	VERY_STRONG // Use for long-term key material
};

std::vector<uint8_t> generateSalt(std::size_t length);

std::vector<uint8_t> generateRandomBytes(std::size_t length,
                                         RandomQuality quality);

template <RandomQuality quality> class SecureUniformRandomNumberGenerator
{
public:
	using result_type = uint64_t;

	static constexpr result_type min();
	static constexpr result_type max();
	result_type operator()();
};

template <RandomQuality quality>
constexpr typename SecureUniformRandomNumberGenerator<quality>::result_type
SecureUniformRandomNumberGenerator<quality>::min()
{
	return std::numeric_limits<uint64_t>::min();
}

template <RandomQuality quality>
constexpr typename SecureUniformRandomNumberGenerator<quality>::result_type
SecureUniformRandomNumberGenerator<quality>::max()
{
	return std::numeric_limits<uint64_t>::max();
}

template <RandomQuality quality>
typename SecureUniformRandomNumberGenerator<quality>::result_type
        SecureUniformRandomNumberGenerator<quality>::
        operator()()
{
	std::vector<uint8_t> bytes =
	        generateRandomBytes(sizeof(uint64_t), quality);
	return *reinterpret_cast<uint64_t const *>(bytes.data());
}

template <RandomQuality quality>
uint64_t
generateRandomNumber(uint64_t min = std::numeric_limits<uint64_t>::min(),
                     uint64_t max = std::numeric_limits<uint64_t>::max())
{
	SecureUniformRandomNumberGenerator<quality> generator;
	std::uniform_int_distribution<uint64_t> distribution(min, max);
	return distribution(generator);
}
}
}
}

#endif
