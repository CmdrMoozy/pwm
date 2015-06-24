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

#include "String.hpp"

#include <algorithm>
#include <locale>

namespace pwm
{
namespace util
{
std::string toLower(const std::string &s)
{
	std::string ret(s);
	std::locale locale;
	std::transform(ret.begin(), ret.end(), ret.begin(), [&locale](char c)
	               {
		               return std::tolower(c, locale);
		       });
	return ret;
}

std::vector<std::string> split(const std::string &s, char d)
{
	std::vector<std::string> components;

	auto start = s.begin();
	auto end = std::find(s.begin(), s.end(), d);
	while(start != s.end())
	{
		if(start != end) components.push_back(std::string(start, end));

		start = end;
		if(start != s.end()) ++start;
		end = std::find(start, s.end(), d);
	}

	return components;
}
}
}
