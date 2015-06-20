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

#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <utility>
#include <vector>

#include <getopt.h>

namespace
{
void printExecutableHelp(char *const *argv,
                         const std::vector<pwm::args::Command> &commands)
{
	std::cout << "Usage: " << argv[0]
	          << " command [options ...] [arguments ...]\n";
	std::cout << "Available commands:\n";
	for(const auto &command : commands)
	{
		std::cout << "\t" << command.name << " - " << command.help
		          << "\n";
	}
}

struct ParsedCommandParameters
{
	pwm::args::OptionsMap options;
	pwm::args::FlagsMap flags;
	pwm::args::ArgumentsMap arguments;

	ParsedCommandParameters() : options(), flags(), arguments()
	{
	}

	ParsedCommandParameters(const ParsedCommandParameters &) = default;
	~ParsedCommandParameters() = default;
	ParsedCommandParameters &
	operator=(const ParsedCommandParameters &) = default;
};

std::string getShortOptions(const pwm::args::Command &command)
{
	std::ostringstream oss;
	for(const auto &option : command.options)
	{
		if(option.shortName != '\0') oss << option.shortName;
		if(!option.flag) oss << ":";
	}
	return oss.str();
}

std::vector<struct option> getLongOptions(const pwm::args::Command &command)
{
	std::vector<struct option> longOptions;
	for(const auto &option : command.options)
	{
		longOptions.push_back({option.name.c_str(),
		                       option.flag
		                               ? no_argument
		                               : !option.defaultVal.empty()
		                                         ? optional_argument
		                                         : required_argument,
		                       nullptr, 1});
	}
	longOptions.push_back({nullptr, 0, nullptr, 0});
	return longOptions;
}

ParsedCommandParameters
parseCommandParameters(int argc, char *const *argv,
                       const pwm::args::Command &command)
{
	std::string shortOptions = getShortOptions(command);
	std::vector<struct option> longOptions = getLongOptions(command);

	// Insert default values for each option, if applicable.
	ParsedCommandParameters parsed;
	for(const auto &option : command.options)
	{
		if(!option.flag && !option.defaultVal.empty())
		{
			parsed.options.insert(
			        std::make_pair(option.name, option.defaultVal));
		}
		else if(option.flag)
		{
			parsed.flags.insert(std::make_pair(option.name, false));
		}
	}

	// Parse all of the option parameters. We Move the argv pointer
	// forward, so we can skip the command argument.
	int skipArgc = argc - 1;
	char *const *skipArgv = &argv[1];
	for(;;)
	{
		int optionIndex = 0;
		int ret = getopt_long(skipArgc, skipArgv, shortOptions.c_str(),
		                      longOptions.data(), &optionIndex);
		if(ret == -1) break;
		const pwm::args::Option &option =
		        command.options[static_cast<decltype(
		                command.options)::size_type>(optionIndex)];

		if(!option.flag)
		{
			parsed.options[option.name] = std::string(optarg);
		}
		else
		{
			parsed.flags[option.name] = true;
		}
	}

	// Parse all of the argument parameters which come after the options.
	if((skipArgc - optind) < static_cast<int>(command.arguments.size()))
		throw std::runtime_error("Missing required arguments.");
	if(((skipArgc - optind) > static_cast<int>(command.arguments.size())) &&
	   !command.lastArgumentVariadic)
	{
		throw std::runtime_error("Too many command arguments.");
	}

	for(int i = optind; i < skipArgc; ++i)
	{
		auto argIdx =
		        static_cast<decltype(command.arguments)::size_type>(
		                i - optind);
		argIdx = argIdx < command.arguments.size()
		                 ? argIdx
		                 : command.arguments.size() - 1;
		parsed.arguments[command.arguments[argIdx].name].push_back(
		        std::string(skipArgv[i]));
	}

	// Verify that we have values for all non-optional parameters.
	for(const auto &option : command.options)
	{
		if(!option.defaultVal.empty()) continue;
		if(option.flag) continue;
		if(parsed.options.find(option.name) == parsed.options.end())
			throw std::runtime_error("Missing required option.");
	}

	return parsed;
}

void printCommandHelp(char *const *argv, const pwm::args::Command &command)
{
	std::cout << "Usage: " << argv[0] << " " << command.name
	          << " [options ...] [arguments ...]\n";

	if(command.options.size() > 0)
	{
		std::cout << "\nOptions:\n";
		for(const auto &option : command.options)
		{
			std::cout << "\t" << option.name << " - "
			          << option.help;
			if(option.flag)
			{
				std::cout << " [Flag, default: off]";
			}
			else if(!option.defaultVal.empty())
			{
				std::cout << " [Default: " << option.defaultVal
				          << "]";
			}
			std::cout << "\n";
		}
	}

	if(command.arguments.size() > 0)
	{
		std::cout << "\nPositional arguments:";
		for(const auto &argument : command.arguments)
		{
			std::cout << "\n\t" << argument.name << " - "
			          << argument.help;
		}
		if(command.lastArgumentVariadic) std::cout << " (One or more)";
		std::cout << "\n";
	}
}
}

namespace pwm
{
namespace args
{
int parseAndExecuteCommand(int argc, char *const *argv,
                           const std::vector<Command> &commands)
{
	// Find the command that we'll parse arguments for.
	if(argc < 2)
	{
		printExecutableHelp(argv, commands);
		return EXIT_FAILURE;
	}
	auto commandIt =
	        std::find_if(commands.begin(), commands.end(),
	                     [&argv](const Command &command)
	                     {
		                     return command.name.compare(argv[1]) == 0;
		             });
	if(commandIt == commands.end())
	{
		printExecutableHelp(argv, commands);
		return EXIT_FAILURE;
	}

	// Try parsing this command's parameters.
	ParsedCommandParameters parameters;
	try
	{
		parameters = parseCommandParameters(argc, argv, *commandIt);
	}
	catch(...)
	{
		printCommandHelp(argv, *commandIt);
		return EXIT_FAILURE;
	}

	// Try parsing the parameters for this command and executing it.
	try
	{
		return commandIt->function(parameters.options, parameters.flags,
		                           parameters.arguments);
	}
	catch(const std::exception &e)
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
