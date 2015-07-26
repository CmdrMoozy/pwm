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

#include "parseOptions.hpp"

#include <sstream>
#include <stdexcept>
#include <utility>
#include <experimental/optional>

#include "pwmc/params/ProgramParameters.hpp"

namespace
{
void insertDefaults(pwm::params::OptionsMap &options,
                    pwm::params::FlagsMap &flags,
                    pwm::params::Command const &command)
{
	for(auto const &option : command.options)
	{
		if(!!option.defaultValue)
		{
			options.insert(std::make_pair(option.name,
			                              *option.defaultValue));
		}
		else if(option.isFlag)
		{
			flags.insert(std::make_pair(option.name, false));
		}
	}
}

void stripHyphens(std::string &parameter)
{
	if(parameter.find("--") == 0)
	{
		parameter = parameter.substr(2);
	}
	else if(parameter.find('-') == 0)
	{
		parameter = parameter.substr(1);
	}
	else
	{
		std::ostringstream oss;
		oss << "Invalid Parameter: '" << parameter << "'.";
		throw std::runtime_error(oss.str());
	}
}

std::experimental::optional<std::string> extractValue(std::string &parameter)
{
	auto idx = parameter.find('=');
	if(idx == std::string::npos) return std::experimental::nullopt;

	std::experimental::optional<std::string> value(
	        parameter.substr(idx + 1));
	parameter = parameter.substr(0, idx);
	return value;
}

struct Parameter
{
	std::string parameter;
	std::experimental::optional<std::string> value;

	Parameter(pwm::params::ProgramParameters const &parameters)
	        : parameter(""), value(std::experimental::nullopt)
	{
		if(parameters.parameters.empty())
		{
			throw std::runtime_error("Cannot construct Parameter "
			                         "from empty ProgramParameters "
			                         "list.");
		}

		parameter = parameters.parameters.front();
		stripHyphens(parameter);
		value = extractValue(parameter);
	}
};

std::string getValue(Parameter const &parameter,
                     pwm::params::Option const &option,
                     pwm::params::ProgramParameters &parameters)
{
	// If we already have a value, just return it.
	if(!!parameter.value) return *parameter.value;

	// Otherwise, if there are no other parameters, we're missing a value.
	if(parameters.parameters.empty())
	{
		std::ostringstream oss;
		oss << "Missing value for option '--" << option.name << "'.";
		throw std::runtime_error(oss.str());
	}

	// Assume the next parameter is the associated value.
	std::string value = parameters.parameters.front();
	parameters.parameters.pop_front();
	return value;
}

void checkAllValuesPresent(pwm::params::OptionsMap const &options,
                           pwm::params::Command const &command)
{
	for(auto const &option : command.options)
	{
		if(option.isFlag) continue;

		if(options.find(option.name) == options.end())
		{
			std::ostringstream oss;
			oss << "No default or specified value for option '--"
			    << option.name << "'.";
			throw std::runtime_error(oss.str());
		}
	}
}
}

namespace pwm
{
namespace params
{
namespace detail
{
std::tuple<OptionsMap, FlagsMap> parseOptions(ProgramParameters &parameters,
                                              Command const &command)
{
	OptionsMap retOptions;
	FlagsMap retFlags;

	// Insert the default value for each option that has one (or false, if
	// the option is a flag). This value will be overwritten if we see the
	// option in the parameters list.
	insertDefaults(retOptions, retFlags, command);

	// Consume as many parameters as possible (until an unknown option).
	while(!parameters.parameters.empty())
	{
		// Get the next parameter, and find its Option. If it is a
		// valid Option, remove it from the program parameters.
		std::experimental::optional<Parameter> parameter;
		try
		{
			parameter.emplace(parameters);
		}
		catch(...)
		{
		}
		if(!parameter) break;
		Option const *option =
		        command.options.find(parameter->parameter);
		if(option == nullptr) break;
		parameters.parameters.pop_front();

		// Insert this option's / flag's value into our return maps.
		if(option->isFlag)
		{
			retFlags[option->name] = true;
		}
		else
		{
			std::string value =
			        getValue(*parameter, *option, parameters);
			retOptions[option->name] = value;
		}
	}

	checkAllValuesPresent(retOptions, command);
	return std::make_tuple(retOptions, retFlags);
}
}
}
}
