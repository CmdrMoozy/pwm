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

#ifndef pwmc_params_Command_HPP
#define pwmc_params_Command_HPP

#include <initializer_list>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "pwmc/params/Argument.hpp"
#include "pwmc/params/Option.hpp"

namespace pwm
{
namespace params
{
typedef std::map<std::string, std::string> OptionsMap;
typedef std::map<std::string, bool> FlagsMap;
typedef std::map<std::string, std::vector<std::string>> ArgumentsMap;

struct Command
{
	std::string name;
	std::string help;
	OptionSet options;
	std::vector<Argument> arguments;
	bool lastArgumentIsVariadic;

	Command(std::string const& n, std::string const& h,
		std::initializer_list<Option> const& o = {},
		std::vector<Argument> const& a = {},
		bool laiv = false);
};

bool operator<(Command const& a, Command const& b);
}
}

#endif
