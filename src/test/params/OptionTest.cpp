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
	        pwm::params::Option("foo", ""),
	        pwm::params::Option("bar", ""),
	        pwm::params::Option("baz", ""),
	        pwm::params::Option("zab", ""),
	        pwm::params::Option("rab", ""),
	        pwm::params::Option("oof", ""),
	        pwm::params::Option("foobar", ""),
	        pwm::params::Option("barbaz", ""),
	        pwm::params::Option("zabrab", ""),
	        pwm::params::Option("raboof", "")};
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

namespace
{
bool findSuccessful(pwm::params::OptionSet const &options,
                    std::string const &parameter,
                    std::string const &expectedName)
{
	pwm::params::Option const *option = options.find(parameter);
	if(option == nullptr) return false;
	return option->name == expectedName;
}
}

TEST_CASE("Test option set finding", "[Parameters]")
{
	pwm::params::OptionSet options{
	        pwm::params::Option("foo", "", 'o', std::experimental::nullopt),
	        pwm::params::Option("bar", "", 'r', std::experimental::nullopt),
	        pwm::params::Option("baz", "", 'z', std::experimental::nullopt,
	                            true),
	        pwm::params::Option("zab", "", 'Z', std::experimental::nullopt,
	                            true),
	        pwm::params::Option("rab", "", 'R', std::experimental::nullopt),
	        pwm::params::Option("oof", "", 'O', std::experimental::nullopt),
	        pwm::params::Option("foobar", "", 'f',
	                            std::experimental::nullopt),
	        pwm::params::Option("barbaz", "", 'b',
	                            std::experimental::nullopt, true),
	        pwm::params::Option("zabrab", "", 'B',
	                            std::experimental::nullopt, true),
	        pwm::params::Option("raboof", "", 'F',
	                            std::experimental::nullopt)};

	CHECK(findSuccessful(options, "foo", "foo"));
	CHECK(findSuccessful(options, "o", "foo"));
	CHECK(findSuccessful(options, "bar", "bar"));
	CHECK(findSuccessful(options, "r", "bar"));
	CHECK(findSuccessful(options, "baz", "baz"));
	CHECK(findSuccessful(options, "z", "baz"));
	CHECK(findSuccessful(options, "zab", "zab"));
	CHECK(findSuccessful(options, "Z", "zab"));
	CHECK(findSuccessful(options, "rab", "rab"));
	CHECK(findSuccessful(options, "R", "rab"));
	CHECK(findSuccessful(options, "oof", "oof"));
	CHECK(findSuccessful(options, "O", "oof"));
	CHECK(findSuccessful(options, "foobar", "foobar"));
	CHECK(findSuccessful(options, "f", "foobar"));
	CHECK(findSuccessful(options, "barbaz", "barbaz"));
	CHECK(findSuccessful(options, "b", "barbaz"));
	CHECK(findSuccessful(options, "zabrab", "zabrab"));
	CHECK(findSuccessful(options, "B", "zabrab"));
	CHECK(findSuccessful(options, "raboof", "raboof"));
	CHECK(findSuccessful(options, "F", "raboof"));

	CHECK(!findSuccessful(options, "foo", "bar"));
	CHECK(!findSuccessful(options, "syn", "syn"));
	CHECK(!findSuccessful(options, "s", "syn"));
	CHECK(!findSuccessful(options, "ack", "ack"));
	CHECK(!findSuccessful(options, "a", "ack"));
	CHECK(!findSuccessful(options, "-", "foobar"));
}
