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

#include "pwmc/args/Argument.hpp"
#include "pwmc/args/Command.hpp"
#include "pwmc/args/Option.hpp"
#include "pwmc/args/parseAndExecuteCommand.hpp"
#include "pwmc/util/OutputSuppressor.hpp"

#include <cstring>
#include <string>
#include <vector>

namespace
{
struct TestParameters
{
	const int parameterCount;
	char **parameters;

	TestParameters(std::vector<std::string> const &p)
	        : parameterCount(p.size()), parameters(nullptr)
	{
		parameters = new char *[p.size()];
		for(int i = 0; i < parameterCount; ++i)
		{
			auto sizei = static_cast<
			        std::vector<std::string>::size_type>(i);
			parameters[sizei] = new char[p[sizei].length() + 1];
			std::strncpy(parameters[sizei], p[sizei].c_str(),
			             p[sizei].length() + 1);
		}
	}

	~TestParameters()
	{
		for(int i = 0; i < parameterCount; ++i)
			delete[] parameters[i];

		delete[] parameters;
	}
};

struct TestFunction
{
	bool executed;
	pwm::args::CommandFunction function;

	TestFunction()
	        : executed(false),
	          function([this](pwm::args::OptionsMap const &,
	                          pwm::args::FlagsMap const &,
	                          pwm::args::ArgumentsMap const &)
	                   {
		                   executed = true;
		           })
	{
	}
};
}

TEST_CASE("Test single command execution", "[Parameters]")
{
	pwm::util::OutputSuppressor suppressor;
	TestParameters params({"ArgsTest", "test"});
	TestFunction function;
	CHECK_FALSE(function.executed);
	REQUIRE_NOTHROW(pwm::args::parseAndExecuteCommand(
	        params.parameterCount, params.parameters,
	        {pwm::args::Command("test", "test", function.function)}));
	CHECK(function.executed);
}

TEST_CASE("Test single command non-execution", "[Parameters]")
{
	pwm::util::OutputSuppressor suppressor;
	TestParameters params({"ArgsTest", "foobar"});
	TestFunction function;
	CHECK_FALSE(function.executed);
	REQUIRE_NOTHROW(pwm::args::parseAndExecuteCommand(
	        params.parameterCount, params.parameters,
	        {pwm::args::Command("test", "test", function.function)}));
	CHECK_FALSE(function.executed);
}

TEST_CASE("Test multiple command execution", "[Parameters]")
{
	pwm::util::OutputSuppressor suppressor;
	TestParameters params({"ArgsTest", "testb"});
	TestFunction functiona;
	TestFunction functionb;
	TestFunction functionc;
	CHECK_FALSE(functiona.executed);
	CHECK_FALSE(functionb.executed);
	CHECK_FALSE(functionc.executed);
	REQUIRE_NOTHROW(pwm::args::parseAndExecuteCommand(
	        params.parameterCount, params.parameters,
	        {pwm::args::Command("testa", "testa", functiona.function),
	         pwm::args::Command("testb", "testb", functionb.function),
	         pwm::args::Command("testc", "testc", functionc.function)}));
	CHECK_FALSE(functiona.executed);
	CHECK(functionb.executed);
	CHECK_FALSE(functionc.executed);
}

TEST_CASE("Test multiple command non-execution", "[Parameters]")
{
	pwm::util::OutputSuppressor suppressor;
	TestParameters params({"ArgsTest", "foobar"});
	TestFunction functiona;
	TestFunction functionb;
	TestFunction functionc;
	CHECK_FALSE(functiona.executed);
	CHECK_FALSE(functionb.executed);
	CHECK_FALSE(functionc.executed);
	REQUIRE_NOTHROW(pwm::args::parseAndExecuteCommand(
	        params.parameterCount, params.parameters,
	        {pwm::args::Command("testa", "testa", functiona.function),
	         pwm::args::Command("testb", "testb", functionb.function),
	         pwm::args::Command("testc", "testc", functionc.function)}));
	CHECK_FALSE(functiona.executed);
	CHECK_FALSE(functionb.executed);
	CHECK_FALSE(functionc.executed);
}
