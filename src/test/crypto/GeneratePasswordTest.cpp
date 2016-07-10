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
#include <cctype>
#include <cstddef>

#include "pwmc/crypto/generatePassword.hpp"

namespace
{
constexpr int TEST_ITERATIONS = 1000;
}

TEST_CASE("Password character exclusion test", "[Crypto]")
{
	for(int i = 0; i < TEST_ITERATIONS; ++i)
	{
		const std::string password = pwm::crypto::generatePassword(
		        {pwm::crypto::PasswordCharacters::LOWERCASE}, 1, 1,
		        {'f'});
		CHECK(std::find_if(password.begin(), password.end(),
		                   [](char const &c) -> bool {
			                   return c == 'f';
			           }) == password.end());
	}
}

TEST_CASE("Passowrd character set test", "[Crypto]")
{
	for(int i = 0; i < TEST_ITERATIONS; ++i)
	{
		const std::string password = pwm::crypto::generatePassword(
		        {pwm::crypto::PasswordCharacters::LOWERCASE,
		         pwm::crypto::PasswordCharacters::UPPERCASE},
		        1, 1);
		CHECK(std::find_if_not(password.begin(), password.end(),
		                       [](char const &c) -> bool {
			                       return std::isalpha(c);
			               }) == password.end());
	}
}

TEST_CASE("Password minimum / maximum length test", "[Crypto]")
{
	constexpr std::size_t TEST_MINIMUM_SIZE = 8;
	constexpr std::size_t TEST_MAXIMUM_SIZE = 10;

	for(int i = 0; i < TEST_ITERATIONS; ++i)
	{
		const std::string password = pwm::crypto::generatePassword(
		        {pwm::crypto::PasswordCharacters::LOWERCASE,
		         pwm::crypto::PasswordCharacters::UPPERCASE,
		         pwm::crypto::PasswordCharacters::NUMBERS,
		         pwm::crypto::PasswordCharacters::SPECIAL},
		        TEST_MINIMUM_SIZE, TEST_MAXIMUM_SIZE);
		CHECK(password.length() >= TEST_MINIMUM_SIZE);
		CHECK(password.length() <= TEST_MAXIMUM_SIZE);
	}
}
