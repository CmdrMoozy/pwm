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

#include "pwmc/params/Argument.hpp"
#include "pwmc/params/Command.hpp"
#include "pwmc/params/ProgramParameters.hpp"
#include "pwmc/params/detail/parseArguments.hpp"

namespace
{
bool valuesArePresent(pwm::params::ArgumentsMap const &arguments,
                      std::string const &name,
                      std::vector<std::string> const &values)
{
	auto it = arguments.find(name);
	if(it == arguments.end()) return false;
	return it->second == values;
}
}

TEST_CASE("Test normal argument parsing", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"oof", "rab", "zab"});

	const pwm::params::Command command("test", "A command for testing.", {},
	                                   {pwm::params::Argument("foo", ""),
	                                    pwm::params::Argument("bar", ""),
	                                    pwm::params::Argument("baz", "")},
	                                   false);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_NOTHROW(arguments = pwm::params::detail::parseArguments(
	                        parameters, command));

	CHECK(arguments.size() == 3);
	CHECK(valuesArePresent(arguments, "foo", {"oof"}));
	CHECK(valuesArePresent(arguments, "bar", {"rab"}));
	CHECK(valuesArePresent(arguments, "baz", {"zab"}));
	CHECK(parameters.parameters.empty());
}

TEST_CASE("Test multiple default values", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"a", "b", "c"});

	const pwm::params::Command command(
	        "test", "A command for testing.", {},
	        {pwm::params::Argument("foo", ""),
	         pwm::params::Argument("bar", ""),
	         pwm::params::Argument("baz", ""),
	         pwm::params::Argument("oof", "", "A"),
	         pwm::params::Argument("rab", "", "B"),
	         pwm::params::Argument("zab", "", "C")},
	        false);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_NOTHROW(arguments = pwm::params::detail::parseArguments(
	                        parameters, command));

	CHECK(arguments.size() == 6);
	CHECK(valuesArePresent(arguments, "foo", {"a"}));
	CHECK(valuesArePresent(arguments, "bar", {"b"}));
	CHECK(valuesArePresent(arguments, "baz", {"c"}));
	CHECK(valuesArePresent(arguments, "oof", {"A"}));
	CHECK(valuesArePresent(arguments, "rab", {"B"}));
	CHECK(valuesArePresent(arguments, "zab", {"C"}));
	CHECK(parameters.parameters.empty());
}

TEST_CASE("Test variadic last argument with default value", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"a"});

	const pwm::params::Command command(
	        "test", "A command for testing.", {},
	        {pwm::params::Argument("foo", ""),
	         pwm::params::Argument("bar", "", "foobar")},
	        true);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_NOTHROW(arguments = pwm::params::detail::parseArguments(
	                        parameters, command));

	CHECK(arguments.size() == 2);
	CHECK(valuesArePresent(arguments, "foo", {"a"}));
	CHECK(valuesArePresent(arguments, "bar", {"foobar"}));
	CHECK(parameters.parameters.empty());
}

TEST_CASE("Test variadic last argument with single value", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"a", "b"});

	const pwm::params::Command command(
	        "test", "A command for testing.", {},
	        {pwm::params::Argument("foo", ""),
	         pwm::params::Argument("bar", "", "foobar")},
	        true);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_NOTHROW(arguments = pwm::params::detail::parseArguments(
	                        parameters, command));

	CHECK(arguments.size() == 2);
	CHECK(valuesArePresent(arguments, "foo", {"a"}));
	CHECK(valuesArePresent(arguments, "bar", {"b"}));
	CHECK(parameters.parameters.empty());
}

TEST_CASE("Test variadic last argument with multiple values", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"a", "b", "c", "d"});

	const pwm::params::Command command(
	        "test", "A command for testing.", {},
	        {pwm::params::Argument("foo", ""),
	         pwm::params::Argument("bar", "", "foobar")},
	        true);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_NOTHROW(arguments = pwm::params::detail::parseArguments(
	                        parameters, command));

	CHECK(arguments.size() == 2);
	CHECK(valuesArePresent(arguments, "foo", {"a"}));
	CHECK(valuesArePresent(arguments, "bar", {"b", "c", "d"}));
	CHECK(parameters.parameters.empty());
}

TEST_CASE("Test extra program parameters", "[Parameters]")
{
	pwm::params::ProgramParameters parameters({"bar", "baz"});

	const pwm::params::Command command("test", "A command for testing.", {},
	                                   {pwm::params::Argument("foo", "")},
	                                   false);

	pwm::params::ArgumentsMap arguments;
	REQUIRE_THROWS(arguments = pwm::params::detail::parseArguments(
	                       parameters, command));
}
