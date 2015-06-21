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

	for(const auto &test : TEST_DATA)
	{
		auto output = pwm::util::split(test.first, TEST_DELIMITER);
		REQUIRE(test.second == output);
	}
}
