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

#include <experimental/optional>

#include "pwmc/params/Option.hpp"

TEST_CASE("Test option default value construction", "[Parameters]")
{
	std::experimental::optional<pwm::params::Option> option;
	CHECK_NOTHROW(option.emplace("foobar", "A test option.", 'f',
	                             std::string("barbaz"), false));
}

TEST_CASE("Test flag option default value construction", "[Parameters]")
{
	std::experimental::optional<pwm::params::Option> option;
	CHECK_THROWS(option.emplace("foobar", "A test option.", 'f',
	                            std::string("barbaz"), true));
}

TEST_CASE("Test default constructed option set iterator equality",
          "[Parameters]")
{
	pwm::params::OptionSetConstIterator a;
	pwm::params::OptionSetConstIterator b;
	CHECK(a == b);
	++a;
	CHECK(a == b);
	++b;
	CHECK(a == b);
}

TEST_CASE("Test option set iterating", "[Parameters]")
{
	const std::initializer_list<pwm::params::Option> optionsList{
	        pwm::params::Option("foo", "foo"),
	        pwm::params::Option("bar", "bar"),
	        pwm::params::Option("baz", "baz"),
	        pwm::params::Option("zab", "zab"),
	        pwm::params::Option("rab", "rab"),
	        pwm::params::Option("oof", "oof"),
	        pwm::params::Option("foobar", "foobar"),
	        pwm::params::Option("barbaz", "barbaz"),
	        pwm::params::Option("zabrab", "zabrab"),
	        pwm::params::Option("raboof", "raboof")};
	pwm::params::OptionSet options(optionsList);
	CHECK(optionsList.size() == options.size());
	CHECK(optionsList.size() ==
	      std::distance(options.begin(), options.end()));

	auto expIt = optionsList.begin();
	for(auto it = options.begin(); it != options.end(); ++it)
	{
		REQUIRE(expIt != optionsList.end());
		CHECK((*expIt).name == (*it).name);
		++expIt;
	}
}
