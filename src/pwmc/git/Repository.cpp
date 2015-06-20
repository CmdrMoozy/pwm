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

#include "Repository.hpp"

namespace
{
std::string discover(const std::string &path)
{
	git_buf buffer = {nullptr, 0, 0};
	pwm::git::checkReturn(
	        git_repository_discover(&buffer, path.c_str(), 0, nullptr));
	return std::string(buffer.ptr);
}
}

namespace pwm
{
namespace git
{
Repository::Repository(const std::string &p)
        : base_type(git_repository_open, discover(p).c_str())
{
}
}
}
