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

TEST_CASE("Test string left trim algorithm", "[String]")
{
	const std::vector<std::pair<std::string, std::string>> TEST_CASES{
	        {"", ""},
	        {"foobar", "foobar"},
	        {"foobar\t\n ", "foobar\t\n "},
	        {"\n\n\nfoobar", "foobar"},
	        {"\t \t \n ", ""},
	        {"\t \t \n foobar", "foobar"},
	        {"foobar \t\n foobar", "foobar \t\n foobar"}};

	for(auto const &test : TEST_CASES)
	{
		std::string result(test.first);
		pwm::util::leftTrim(result);
		CHECK(test.second == result);
	}
}

TEST_CASE("Test string right trim algorithm", "[String]")
{
	const std::vector<std::pair<std::string, std::string>> TEST_CASES{
	        {"", ""},
	        {"foobar", "foobar"},
	        {"foobar\t\n ", "foobar"},
	        {"foobar\n\n\n", "foobar"},
	        {"\n\n\nfoobar", "\n\n\nfoobar"},
	        {"\t \t \n ", ""},
	        {"foobar\t \t \n ", "foobar"},
	        {"foobar \t\n foobar", "foobar \t\n foobar"}};

	for(auto const &test : TEST_CASES)
	{
		std::string result(test.first);
		pwm::util::rightTrim(result);
		CHECK(test.second == result);
	}
}

TEST_CASE("Test string trim algorithm", "[String]")
{
	const std::vector<std::pair<std::string, std::string>> TEST_CASES{
	        {"", ""},
	        {"foobar", "foobar"},
	        {"foobar\t\n ", "foobar"},
	        {"foobar\n\n\n", "foobar"},
	        {"\n\n\nfoobar", "foobar"},
	        {"\t \t \n ", ""},
	        {"foobar\t \t \n ", "foobar"},
	        {"foobar \t\n foobar", "foobar \t\n foobar"}};

	for(auto const &test : TEST_CASES)
	{
		std::string result(test.first);
		pwm::util::trim(result);
		CHECK(test.second == result);
	}
}

namespace
{
struct RemoveRepeatedCharacterTestCase
{
	std::string input;
	char character;
	std::string expected;

	RemoveRepeatedCharacterTestCase(std::string const &i, char c,
	                                std::string const &e)
	        : input(i), character(c), expected(e)
	{
	}
};
}

TEST_CASE("Test repeated character removal", "[String]")
{
	const std::vector<RemoveRepeatedCharacterTestCase> TEST_CASES{
	        {"", ' ', ""},
	        {"abcdefghijklmnop", 'g', "abcdefghijklmnop"},
	        {"/foo/bar//baz/test/foobar//", '/',
	         "/foo/bar/baz/test/foobar/"},
	        {"//////////", '/', "/"},
	        {"/", '/', "/"}};

	for(auto const &test : TEST_CASES)
	{
		std::string result(test.input);
		pwm::util::removeRepeatedCharacters(result, test.character);
		CHECK(test.expected == result);
	}
}
