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

#include "parseArguments.hpp"

#include <sstream>
#include <stdexcept>

#include "pwmc/params/ProgramParameters.hpp"

namespace pwm
{
namespace params
{
namespace detail
{
ArgumentsMap parseArguments(ProgramParameters &parameters,
                            Command const &command)
{
	ArgumentsMap retArguments;

	// Grab exactly one value for each argument, or until the parameters
	// list is empty.
	decltype(command.arguments)::const_iterator lastUnparsed;
	for(auto it = command.arguments.cbegin();
	    it != command.arguments.cend(); ++it)
	{
		lastUnparsed = it;
		if(parameters.parameters.empty()) break;
		retArguments[it->name].push_back(parameters.parameters.front());
		parameters.parameters.pop_front();
		++lastUnparsed;
	}

	// If there were arguments we didn't get values for, insert their
	// default values.
	for(; lastUnparsed != command.arguments.cend(); ++lastUnparsed)
	{
		if(!!lastUnparsed->defaultValue)
		{
			retArguments[lastUnparsed->name].push_back(
			        *lastUnparsed->defaultValue);
		}
		else
		{
			std::ostringstream oss;
			oss << "No specified or default value for argument '"
			    << lastUnparsed->name << "'.";
			throw std::runtime_error(oss.str());
		}
	}

	// If the last argument is variadic, and there are any other parameters
	// left over, add them all to that last argument.
	if(command.lastArgumentIsVariadic)
	{
		std::string const &name = command.arguments.rbegin()->name;
		while(!parameters.parameters.empty())
		{
			retArguments[name].push_back(
			        parameters.parameters.front());
			parameters.parameters.pop_front();
		}
	}

	// If there are any parameters left over, we have a problem.
	if(!parameters.parameters.empty())
	{
		throw std::runtime_error("Found unused program parameters "
		                         "after parsing command parameters.");
	}

	return retArguments;
}
}
}
}
