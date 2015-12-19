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

#include "Path.hpp"

#include <algorithm>
#include <locale>
#include <sstream>
#include <stdexcept>

#include <bdrck/algorithm/String.hpp>

namespace
{
bool isValidPath(std::string const &path)
{
	auto it = std::find_if_not(path.begin(), path.end(),
	                           [](char const &c) -> bool
	                           {
		                           std::locale locale;
		                           return std::isalpha(c, locale) ||
		                                  std::isdigit(c, locale) ||
		                                  (c == '/') || (c == '\\');
		                   });
	return it == path.end();
}

void normalizeSeparators(std::string &path)
{
	std::replace_if(path.begin(), path.end(),
	                [](char const &c) -> bool
	                {
		                return c == '\\';
		        },
	                '/');
}

void trimSeparators(std::string &path)
{
	bdrck::algorithm::string::trim(path, [](char const &c) -> bool
	                               {
		                               return c == '/';
		                       });
}
}

namespace pwm
{
namespace repository
{
Path::Path(std::string const &p) : path(p)
{
	if(!isValidPath(path))
	{
		std::ostringstream oss;
		oss << "Invalid repository path: '" << path << "'.";
		throw std::runtime_error(oss.str());
	}

	normalizeSeparators(path);
	trimSeparators(path);
	bdrck::algorithm::string::removeRepeatedCharacters(path, '/');
}

std::string const &Path::getPath() const
{
	return path;
}

std::ostream &operator<<(std::ostream &out, Path const &path)
{
	out << path.getPath();
	return out;
}
}
}
