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

#ifndef pwmc_args_Command_HPP
#define pwmc_args_Command_HPP

#include <map>
#include <string>
#include <vector>

#include "pwmc/args/Argument.hpp"
#include "pwmc/args/Option.hpp"

namespace pwm
{
namespace args
{
typedef std::map<std::string, std::string> OptionsMap;
typedef std::map<std::string, bool> FlagsMap;
typedef std::map<std::string, std::vector<std::string>> ArgumentsMap;

typedef void (*CommandFunction)(const OptionsMap&, const FlagsMap&, const ArgumentsMap&);

struct Command
{
	std::string name;
	std::string help;
	CommandFunction function;
	std::vector<Option> options;
	std::vector<Argument> arguments;
	bool lastArgumentVariadic;

	Command(const std::string& n, const std::string &h, CommandFunction f,
		const std::vector<Option> o = {},
		const std::vector<Argument> a = {},
		bool lav = false);
};
}
}

#endif
