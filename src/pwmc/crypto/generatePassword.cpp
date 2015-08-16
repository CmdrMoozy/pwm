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

#include "generatePassword.hpp"

#include <algorithm>
#include <cassert>
#include <iterator>
#include <map>
#include <sstream>
#include <stdexcept>

#include "pwmc/crypto/Util.hpp"

namespace
{
// clang-format off
const std::map<pwm::crypto::PasswordCharacters, std::vector<char>> PASSWORD_CHARACTERS{
	{pwm::crypto::PasswordCharacters::LOWERCASE, {
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
		'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
		'y', 'z'
	}},
	{pwm::crypto::PasswordCharacters::UPPERCASE, {
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
		'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
		'Y', 'Z'
	}},
	{pwm::crypto::PasswordCharacters::NUMBERS, {
		'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
	}},
	{pwm::crypto::PasswordCharacters::SPECIAL, {
		'`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
		'-', '_', '=', '+', '/', '[', '{', ']', '}', '\\', '|', ';',
		':', '\'', '"', ',', '<', '.', '>', '?'
	}}
};
// clang-format on
}

namespace pwm
{
namespace crypto
{
std::string generatePassword(std::vector<PasswordCharacters> characters,
                             std::size_t minimumLength,
                             std::size_t maximumLength,
                             std::set<char> excludedCharacters)
{
	std::vector<char> characterSet;
	{
		std::set<char> includedCharacters;
		for(auto const &c : characters)
		{
			auto it = PASSWORD_CHARACTERS.find(c);
			assert(it != PASSWORD_CHARACTERS.end());
			includedCharacters.insert(it->second.begin(),
			                          it->second.end());
		}
		std::set_difference(
		        includedCharacters.begin(), includedCharacters.end(),
		        excludedCharacters.begin(), excludedCharacters.end(),
		        std::back_inserter(characterSet));
	}

	if(characterSet.empty())
	{
		throw std::runtime_error("Cannot generate a password from an "
		                         "empty character set.");
	}

	static_assert(
	        sizeof(std::size_t) <= sizeof(uint64_t),
	        "size_t must fit inside uint64_t for these casts to work.");
	std::size_t length = static_cast<std::size_t>(
	        util::generateRandomNumber<util::RandomQuality::STRONG>(
	                static_cast<uint64_t>(minimumLength),
	                static_cast<uint64_t>(maximumLength)));

	std::ostringstream oss;
	for(std::size_t i = 0; i < length; ++i)
	{
		using IndexType = std::vector<char>::size_type;
		static_assert(sizeof(IndexType) <= sizeof(uint64_t),
		              "Index type must fit inside uint64_t for these "
		              "casts to work.");
		IndexType idx = static_cast<IndexType>(
		        util::generateRandomNumber<util::RandomQuality::STRONG>(
		                static_cast<uint64_t>(0),
		                static_cast<uint64_t>(characterSet.size() -
		                                      1)));
		oss << characterSet[idx];
	}
	return oss.str();
}
}
}
