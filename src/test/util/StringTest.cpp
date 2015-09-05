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

#include <string>
#include <utility>
#include <vector>

#include "pwmc/util/String.hpp"

TEST_CASE("Test string lowercasing algorithm", "[String]")
{
	const std::vector<std::pair<std::string, std::string>> TEST_CASES{
	        {"", ""},
	        {" 1234567890 !@#$%^&*() -= \\/+_",
	         " 1234567890 !@#$%^&*() -= \\/+_"},
	        {"abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"},
	        {"ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz"},
	        {"17#@&$*dAcJfHssdkFKdjsS(9", "17#@&$*dacjfhssdkfkdjss(9"},
	        {"   \t   ", "   \t   "}};

	for(auto const &test : TEST_CASES)
	{
		auto output = pwm::util::toLower(test.first);
		CHECK(test.second == output);
	}
}

TEST_CASE("Test string split algorithm", "[String]")
{
	const char TEST_DELIMITER = ',';
	const std::vector<std::pair<std::string, std::vector<std::string>>>
	        TEST_DATA = {{"", {}},
	                     {",,,,,,,,", {}},
	                     {"foobar", {"foobar"}},
	                     {",,foobar", {"foobar"}},
	                     {"foobar,,", {"foobar"}},
	                     {",,,,foobar,,,,", {"foobar"}},
	                     {",,,,foo,,,,bar,,,,", {"foo", "bar"}},
	                     {"f,o,o,b,a,r", {"f", "o", "o", "b", "a", "r"}}};

	for(auto const &test : TEST_DATA)
	{
		auto output = pwm::util::split(test.first, TEST_DELIMITER);
		CHECK(test.second == output);
	}
}

namespace
{
struct JoinTestCase
{
	std::vector<std::string> input;
	std::string delimiter;
	std::string expected;

	JoinTestCase(std::vector<std::string> const &i, std::string const &d,
	             std::string const &e)
	        : input(i), delimiter(d), expected(e)
	{
	}
};
}

TEST_CASE("Test string join algorithm", "[String]")
{
	const std::vector<JoinTestCase> TEST_CASES{
	        {{"foo", "bar", "baz"}, " ", "foo bar baz"},
	        {{}, "foobar", ""},
	        {{"", "", ""}, ",", ",,"},
	        {{"foo", "bar", "baz"}, "", "foobarbaz"}};

	for(auto const &test : TEST_CASES)
	{
		std::string output = pwm::util::join(
		        test.input.begin(), test.input.end(), test.delimiter);
		CHECK(test.expected == output);
	}
}
