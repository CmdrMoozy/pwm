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

#include <tuple>
#include <experimental/optional>

#include "pwmc/params/Command.hpp"
#include "pwmc/params/Option.hpp"
#include "pwmc/params/ProgramParameters.hpp"
#include "pwmc/params/detail/parseOptions.hpp"

namespace
{
const pwm::params::Command
        TEST_COMMAND("test", "A command for testing purposes.",
                     pwm::params::CommandFunction(),
                     {pwm::params::Option("flaga", "", 'a',
                                          std::experimental::nullopt, true),
                      pwm::params::Option("optiona", "", 'A'),
                      pwm::params::Option("flagb", "", 'b',
                                          std::experimental::nullopt, true),
                      pwm::params::Option("optionb", "", 'B', "bdefault"),
                      pwm::params::Option("flagc", "", 'c',
                                          std::experimental::nullopt, true),
                      pwm::params::Option("optionc", "", 'C')},
                     {}, false);

bool optionValueCorrect(std::string const &name,
                        std::string const &expectedValue,
                        std::tuple<pwm::params::OptionsMap,
                                   pwm::params::FlagsMap> const &parsed)
{
	auto it = std::get<0>(parsed).find(name);
	if(it == std::get<0>(parsed).end()) return false;
	return it->second == expectedValue;
}

bool flagValueCorrect(std::string const &name, bool expectedValue,
                      std::tuple<pwm::params::OptionsMap,
                                 pwm::params::FlagsMap> const &parsed)
{
	auto it = std::get<1>(parsed).find(name);
	if(it == std::get<1>(parsed).end()) return false;
	return it->second == expectedValue;
}
}

TEST_CASE("Test mixed name option parsing", "[Parameters]")
{
	pwm::params::ProgramParameters parameters(
	        {"--flaga", "--optiona", "foobar", "--flagb", "-B", "barbaz",
	         "-c", "--optionc", "foobaz"});

	std::tuple<pwm::params::OptionsMap, pwm::params::FlagsMap> parsed;
	REQUIRE_NOTHROW(parsed = pwm::params::detail::parseOptions(
	                        parameters, TEST_COMMAND));
	CHECK(parameters.parameters.size() == 0);

	CHECK(flagValueCorrect("flaga", true, parsed));
	CHECK(optionValueCorrect("optiona", "foobar", parsed));
	CHECK(flagValueCorrect("flagb", true, parsed));
	CHECK(optionValueCorrect("optionb", "barbaz", parsed));
	CHECK(flagValueCorrect("flagc", true, parsed));
	CHECK(optionValueCorrect("optionc", "foobaz", parsed));
}

TEST_CASE("Test missing options after parsing", "[Parameters]")
{
	pwm::params::ProgramParameters parameters(
	        {"--flaga", "-b", "--optiona", "foobar"});

	std::tuple<pwm::params::OptionsMap, pwm::params::FlagsMap> parsed;
	REQUIRE_THROWS(parsed = pwm::params::detail::parseOptions(
	                       parameters, TEST_COMMAND));
	CHECK(parameters.parameters.size() == 0);
}

TEST_CASE("Test defaulted option values", "[Parameters]")
{
	pwm::params::ProgramParameters parameters(
	        {"--flaga", "-c", "--optiona", "foobar", "-C", "barbaz"});

	std::tuple<pwm::params::OptionsMap, pwm::params::FlagsMap> parsed;
	REQUIRE_NOTHROW(parsed = pwm::params::detail::parseOptions(
	                        parameters, TEST_COMMAND));
	CHECK(parameters.parameters.size() == 0);

	CHECK(flagValueCorrect("flaga", true, parsed));
	CHECK(optionValueCorrect("optiona", "foobar", parsed));
	CHECK(flagValueCorrect("flagb", false, parsed));
	CHECK(optionValueCorrect("optionb", "bdefault", parsed));
	CHECK(flagValueCorrect("flagc", true, parsed));
	CHECK(optionValueCorrect("optionc", "barbaz", parsed));
}

TEST_CASE("Test mixed value specification option parsing", "[Parameters]")
{
	pwm::params::ProgramParameters parameters(
	        {"-A=foobar", "--optionb", "barbaz", "--optionc=foobaz"});

	std::tuple<pwm::params::OptionsMap, pwm::params::FlagsMap> parsed;
	REQUIRE_NOTHROW(parsed = pwm::params::detail::parseOptions(
	                        parameters, TEST_COMMAND));
	CHECK(parameters.parameters.size() == 0);

	CHECK(flagValueCorrect("flaga", false, parsed));
	CHECK(optionValueCorrect("optiona", "foobar", parsed));
	CHECK(flagValueCorrect("flagb", false, parsed));
	CHECK(optionValueCorrect("optionb", "barbaz", parsed));
	CHECK(flagValueCorrect("flagc", false, parsed));
	CHECK(optionValueCorrect("optionc", "foobaz", parsed));
}

TEST_CASE("Test arguments left alone during option parsing", "[Parameters]")
{
	pwm::params::ProgramParameters parameters(
	        {"--flaga", "--optiona", "foobar", "--optionc", "barbaz",
	         "someargument", "-b", "--flagc", "--optionb", "foobaz"});

	std::tuple<pwm::params::OptionsMap, pwm::params::FlagsMap> parsed;
	REQUIRE_NOTHROW(parsed = pwm::params::detail::parseOptions(
	                        parameters, TEST_COMMAND));
	CHECK(parameters.parameters.size() == 5);

	CHECK(flagValueCorrect("flaga", true, parsed));
	CHECK(optionValueCorrect("optiona", "foobar", parsed));
	CHECK(flagValueCorrect("flagb", false, parsed));
	CHECK(optionValueCorrect("optionb", "bdefault", parsed));
	CHECK(flagValueCorrect("flagc", false, parsed));
	CHECK(optionValueCorrect("optionc", "barbaz", parsed));
}
