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

#include "parseCommand.hpp"

#include "pwmc/params/ProgramParameters.hpp"

namespace pwm
{
namespace params
{
namespace detail
{
std::set<Command>::const_iterator
parseCommand(ProgramParameters &parameters, std::set<Command> const &commands)
{
	if(parameters.parameters.empty()) return commands.cend();
	Command search(parameters.parameters.front(), "");
	auto ret = commands.find(search);
	if(ret != commands.cend()) parameters.parameters.pop_front();
	return ret;
}
}
}
}
