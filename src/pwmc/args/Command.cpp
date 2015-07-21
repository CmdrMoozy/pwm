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

#include "Command.hpp"

#include <stdexcept>

namespace pwm
{
namespace args
{
Command::Command(const std::string &n, const std::string &h, CommandFunction f,
                 const std::vector<Option> o, const std::vector<Argument> a,
                 bool lav)
        : name(n),
          help(h),
          function(f),
          options(o),
          arguments(a),
          lastArgumentVariadic(lav)
{
	// Only the last argument can have a default value.
	if(!arguments.empty())
	{
		for(auto const &argument : arguments)
		{
			if(&argument == &arguments.back()) continue;
			if(!!argument.defaultVal)
			{
				throw std::runtime_error(
				        "Only the last command argument can "
				        "have a default value.");
			}
		}
	}
}
}
}
