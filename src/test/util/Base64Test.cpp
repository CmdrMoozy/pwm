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

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "pwmc/util/Base64.hpp"

namespace
{
const std::vector<std::pair<std::string, std::string>> TEST_VECTORS{
        {"", ""},
        {"f", "Zg=="},
        {"fo", "Zm8="},
        {"foo", "Zm9v"},
        {"foob", "Zm9vYg=="},
        {"fooba", "Zm9vYmE="},
        {"foobar", "Zm9vYmFy"}};
}

TEST_CASE("Test base-64 encoding", "[Base64]")
{
	for(auto const &vector : TEST_VECTORS)
	{
		std::string encoded = pwm::util::encodeBase64(
		        vector.first.data(), vector.first.length());
		CHECK(vector.second == encoded);
	}
}

TEST_CASE("Test base-64 decoding", "[Base64]")
{
	for(auto const &vector : TEST_VECTORS)
	{
		std::vector<uint8_t> decoded =
		        pwm::util::decodeBase64(vector.second);
		std::string decodedString(
		        reinterpret_cast<char const *>(decoded.data()),
		        decoded.size());
		CHECK(vector.first == decodedString);
	}
}
