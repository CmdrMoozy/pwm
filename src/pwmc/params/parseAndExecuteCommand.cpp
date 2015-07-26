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

#include "parseAndExecuteCommand.hpp"

#include "pwmc/params/ProgramParameters.hpp"
#include "pwmc/params/detail/parseArguments.hpp"
#include "pwmc/params/detail/parseCommand.hpp"
#include "pwmc/params/detail/parseOptions.hpp"

#include <cstdlib>
#include <iostream>
#include <stdexcept>
#include <string>
#include <tuple>

namespace
{
void printProgramHelp(std::string const &program,
                      std::set<pwm::params::Command> const &commands)
{
	std::cout << "Usage: " << program
	          << " command [options ...] [arguments ...]\n";
	std::cout << "Available commands:\n";
	for(auto const &command : commands)
	{
		std::cout << "\t" << command.name << " - " << command.help
		          << "\n";
	}
}

void printCommandHelp(std::string const &program,
                      pwm::params::Command const &command)
{
	std::cout << "Usage: " << program << " " << command.name
	          << " [options ...] [arguments ...]\n";

	if(command.options.size() > 0)
	{
		std::cout << "\nOptions:\n";
		for(auto const &option : command.options)
		{
			std::cout << "\t--" << option.name;
			if(!!option.shortName)
				std::cout << ", -" << *option.shortName;
			std::cout << " - " << option.help;

			if(option.isFlag)
			{
				std::cout << " [Flag, default: off]";
			}
			else if(!!option.defaultValue)
			{
				std::cout
				        << " [Default: " << *option.defaultValue
				        << "]";
			}
			std::cout << "\n";
		}
	}

	if(command.arguments.size() > 0)
	{
		std::cout << "\nPositional arguments:";
		for(auto const &argument : command.arguments)
		{
			std::cout << "\n\t" << argument.name << " - "
			          << argument.help;
			if(!!argument.defaultValue)
			{
				std::cout << " [Default: "
				          << *argument.defaultValue << "]";
			}
		}
		if(command.lastArgumentIsVariadic)
			std::cout << " [One or more]";
		std::cout << "\n";
	}
}
}

namespace pwm
{
namespace params
{
int parseAndExecuteCommand(int argc, char const *const *argv,
                           std::set<Command> const &commands)
{
	ProgramParameters parameters(argc, argv);

	// First, figure out which command we'll be parsing parameters for.
	auto commandIt = detail::parseCommand(parameters, commands);
	if(commandIt == commands.cend())
	{
		printProgramHelp(argv[0], commands);
		return EXIT_FAILURE;
	}

	// Parse this command's options and arguments.
	std::tuple<OptionsMap, FlagsMap> options;
	ArgumentsMap arguments;
	try
	{
		options = detail::parseOptions(parameters, *commandIt);
		arguments = detail::parseArguments(parameters, *commandIt);
	}
	catch(std::exception const &e)
	{
		std::cerr << "ERROR: " << e.what() << "\n";
		printCommandHelp(argv[0], *commandIt);
		return EXIT_FAILURE;
	}
	catch(...)
	{
		std::cerr << "ERROR: Unknown exception\n";
		printCommandHelp(argv[0], *commandIt);
		return EXIT_FAILURE;
	}

	// Execute the user-provided function.
	try
	{
		if(commandIt->function)
		{
			commandIt->function(std::get<0>(options),
			                    std::get<1>(options), arguments);
		}
		return EXIT_SUCCESS;
	}
	catch(std::exception const &e)
	{
		std::cerr << "ERROR: " << e.what() << "\n";
	}
	catch(...)
	{
		std::cerr << "ERROR: Unknown exception.\n";
	}

	return EXIT_FAILURE;
}
}
}
