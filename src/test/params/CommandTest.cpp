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

#include "pwmc/params/Argument.hpp"
#include "pwmc/params/Command.hpp"

TEST_CASE("Test command construction with valid defaulted arguments",
          "[Parameters]")
{
	std::experimental::optional<pwm::params::Command> command;
	REQUIRE_NOTHROW(command.emplace(
	        "test", "A test command.", pwm::params::CommandFunction(),
	        std::initializer_list<pwm::params::Option>({}),
	        std::vector<pwm::params::Argument>(
	                {pwm::params::Argument("foo", "foo"),
	                 pwm::params::Argument("bar", "bar", "foobar"),
	                 pwm::params::Argument("baz", "baz", "barbaz")}),
	        false));
}

TEST_CASE("Test command construction with invalid defaulted arguments",
          "[Parameters]")
{
	std::experimental::optional<pwm::params::Command> command;
	REQUIRE_THROWS(command.emplace(
	        "test", "A test command.", pwm::params::CommandFunction(),
	        std::initializer_list<pwm::params::Option>({}),
	        std::vector<pwm::params::Argument>(
	                {pwm::params::Argument("foo", "foo"),
	                 pwm::params::Argument("bar", "bar", "foobar"),
	                 pwm::params::Argument("baz", "baz")}),
	        false));
}
