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

#include <set>

#include "pwmc/params/Command.hpp"
#include "pwmc/params/ProgramParameters.hpp"
#include "pwmc/params/detail/parseCommand.hpp"

TEST_CASE("Test invalid command", "[Parameters]")
{
	std::set<pwm::params::Command> commands;
	commands.emplace("foo", "foo").first;
	commands.emplace("bar", "bar").first;
	commands.emplace("baz", "baz").first;

	pwm::params::ProgramParameters parameters{"biff", "foo", "bar", "baz"};
	REQUIRE(4 == parameters.parameters.size());
	CHECK(commands.cend() ==
	      pwm::params::detail::parseCommand(parameters, commands));
	CHECK(4 == parameters.parameters.size());
}

TEST_CASE("Test command with no arguments", "[Parameters]")
{
	std::set<pwm::params::Command> commands;
	commands.emplace("foo", "foo").first;
	const auto barIt = commands.emplace("bar", "bar").first;
	commands.emplace("baz", "baz").first;

	pwm::params::ProgramParameters parameters{"bar"};
	REQUIRE(1 == parameters.parameters.size());
	CHECK(barIt == pwm::params::detail::parseCommand(parameters, commands));
	CHECK(0 == parameters.parameters.size());
}

TEST_CASE("Test command with arguments", "[Parameters]")
{
	std::set<pwm::params::Command> commands;
	commands.emplace("foo", "foo").first;
	commands.emplace("bar", "bar").first;
	const auto bazIt = commands.emplace("baz", "baz").first;

	pwm::params::ProgramParameters parameters{"baz", "foo", "bar", "baz"};
	REQUIRE(4 == parameters.parameters.size());
	CHECK(bazIt == pwm::params::detail::parseCommand(parameters, commands));
	CHECK(3 == parameters.parameters.size());
}
