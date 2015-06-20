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

#include "pwmc/git/checkReturn.hpp"

namespace
{
std::string discover(const std::string &p)
{
	git_buf buffer = {nullptr, 0, 0};
	pwm::git::checkReturn(
	        git_repository_discover(&buffer, p.c_str(), 0, nullptr));
	return std::string(buffer.ptr);
}

std::string getRepositoryConstructPath(const std::string &p,
                                       pwm::git::RepositoryCreateMode c,
                                       bool ab)
{
	try
	{
		return discover(p);
	}
	catch(...)
	{
		if(c == pwm::git::RepositoryCreateMode::NO_CREATE) throw;
		if(!ab && (c == pwm::git::RepositoryCreateMode::CREATE_BARE))
			throw;

		git_repository *repo;
		pwm::git::checkReturn(git_repository_init(
		        &repo, p.c_str(),
		        c == pwm::git::RepositoryCreateMode::CREATE_NORMAL
		                ? 0
		                : 1));
		git_repository_free(repo);
		return p;
	}
}
}

namespace pwm
{
namespace git
{
Repository::Repository(const std::string &p, RepositoryCreateMode c, bool ab)
        : base_type(git_repository_open,
                    getRepositoryConstructPath(p, c, ab).c_str())
{
}
}
}
