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

#ifndef pwmc_params_ProgramParameters_HPP
#define pwmc_params_ProgramParameters_HPP

#include <initializer_list>
#include <list>
#include <string>

namespace pwm
{
namespace params
{
struct ProgramParameters
{
	std::list<std::string> parameters;

	explicit ProgramParameters(std::list<std::string> const& p);
	explicit ProgramParameters(std::initializer_list<std::string> const& p);
	ProgramParameters(int argc, char const* const* argv);
};
}
}

#endif