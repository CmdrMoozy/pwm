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

#include <sstream>
#include <string>

#include "pwmc/config/Configuration.hpp"
#include "pwmc/config/deserializeConfiguration.hpp"
#include "pwmc/config/serializeConfiguration.hpp"

TEST_CASE("Test configuration {de,}serialization round trip", "[Configuration]")
{
	pwm::config::ConfigurationData original;
	original.data = {{pwm::config::Key("foo.bar.baz"), "foo"},
	                 {pwm::config::Key("foo.bar.foobar"), "bar"},
	                 {pwm::config::Key("blah"), "baz"}};

	std::string serialized =
	        pwm::config::serializeConfiguration(original, false);
	std::string serializedFormatted =
	        pwm::config::serializeConfiguration(original, true);

	std::istringstream serializedIn(serialized);
	pwm::config::ConfigurationData deserialized =
	        pwm::config::deserializeConfiguration(serializedIn);
	std::istringstream serializedFormattedIn(serializedFormatted);
	pwm::config::ConfigurationData deserializedFormatted =
	        pwm::config::deserializeConfiguration(serializedFormattedIn);

	REQUIRE(original.data == deserialized.data);
	REQUIRE(original.data == deserializedFormatted.data);
}
