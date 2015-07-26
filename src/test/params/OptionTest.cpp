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
	CHECK_NOTHROW(option = pwm::params::Option::required(
	                      "foobar", "A test option.", 'f', "barbaz"));
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
	        pwm::params::Option::required("foo", ""),
	        pwm::params::Option::required("bar", ""),
	        pwm::params::Option::required("baz", ""),
	        pwm::params::Option::required("zab", ""),
	        pwm::params::Option::required("rab", ""),
	        pwm::params::Option::required("oof", ""),
	        pwm::params::Option::required("foobar", ""),
	        pwm::params::Option::required("barbaz", ""),
	        pwm::params::Option::required("zabrab", ""),
	        pwm::params::Option::required("raboof", "")};
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
	        pwm::params::Option::required("foo", "", 'o'),
	        pwm::params::Option::required("bar", "", 'r'),
	        pwm::params::Option::flag("baz", "", 'z'),
	        pwm::params::Option::flag("zab", "", 'Z'),
	        pwm::params::Option::required("rab", "", 'R'),
	        pwm::params::Option::required("oof", "", 'O'),
	        pwm::params::Option::required("foobar", "", 'f'),
	        pwm::params::Option::flag("barbaz", "", 'b'),
	        pwm::params::Option::flag("zabrab", "", 'B'),
	        pwm::params::Option::required("raboof", "", 'F')};

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
