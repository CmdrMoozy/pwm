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

#include "list.hpp"

#include <iostream>
#include <utility>

#include <boost/optional/optional.hpp>

#include <git2.h>

#include <bdrck/git/Commit.hpp>
#include <bdrck/git/Oid.hpp>
#include <bdrck/git/Tree.hpp>
#include <bdrck/git/Util.hpp>

#include "pwmc/repository/EncryptionHeader.hpp"

namespace
{
/**
 * Returns the current tree from the repository. Either the HEAD tree, or the
 * empty tree if no commits exist yet.
 */
bdrck::git::Tree getCurrentTree(bdrck::git::Repository &repository)
{
	boost::optional<bdrck::git::Oid> oid =
	        bdrck::git::revspecToOid("HEAD", repository);
	if(!!oid)
	{
		return bdrck::git::Tree{bdrck::git::Commit{repository, *oid}};
	}
	else
	{
		return bdrck::git::Tree{repository,
		                        bdrck::git::getEmptyTreeOid()};
	}
}
}

namespace pwm
{
namespace repository
{
void list(Repository &repository, Path const &path,
          std::function<bool(Path const &)> const &)
{
	bdrck::git::Tree tree = getCurrentTree(*repository.repository);
	tree.walk(
	        [&path](std::string const &p) -> bool {
		        // Don't print out the encryption header's path.
		        if(p == ENCRYPTION_HEADER_RELATIVE_PATH) return true;

		        // If this path doesn't start with the user-specified
		        // path
		        // query, don't print it out.
		        if(p.find(path.getRelativePath()) != 0) return true;

		        std::cout << p << "\n";
		        return true;
		},
	        GIT_FILEMODE_BLOB | GIT_FILEMODE_BLOB_EXECUTABLE |
	                GIT_FILEMODE_LINK);
}
}
}
