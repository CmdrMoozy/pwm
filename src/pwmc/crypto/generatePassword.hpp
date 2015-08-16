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

#ifndef pwmc_crypto_generatePassword_HPP
#define pwmc_crypto_generatePassword_HPP

#include <cstddef>
#include <cstdint>
#include <set>
#include <string>
#include <vector>

namespace pwm
{
namespace crypto
{
enum class PasswordCharacters
{
	LOWERCASE,
	UPPERCASE,
	NUMBERS,
	SPECIAL
};

std::string generatePassword(
        std::vector<PasswordCharacters> characters =
                {PasswordCharacters::LOWERCASE, PasswordCharacters::UPPERCASE,
                 PasswordCharacters::NUMBERS, PasswordCharacters::SPECIAL},
        std::size_t minimumLength = 8, std::size_t maximumLength = 32,
        std::set<char> excludedCharacters = {});
}
}

#endif
