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

#include "Option.hpp"

#include <stdexcept>

namespace pwm
{
namespace params
{
Option::Option(std::string const &n, std::string const &h,
               std::experimental::optional<char> const &sn,
               std::experimental::optional<std::string> const &dv, bool f)
        : name(n), help(h), shortName(sn), defaultValue(dv), flag(f)
{
	if(flag && !!defaultValue)
	{
		throw std::runtime_error(
		        "Flag options cannot have default values.");
	}
}

Option::Option(std::string const &n, std::string const &h,
               std::experimental::optional<char> const &sn,
               std::string const &dv)
        : Option(n, h, sn, dv, false)
{
}
}
}