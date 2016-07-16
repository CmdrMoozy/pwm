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

#include "IO.hpp"

#include <algorithm>
#include <fstream>
#include <iterator>
#include <sstream>
#include <stdexcept>

#include <bdrck/fs/Util.hpp>
#include <bdrck/git/Commit.hpp>
#include <bdrck/git/Index.hpp>
#include <bdrck/git/StrArray.hpp>

namespace
{
std::string getPasswordChangeMessage(pwm::repository::Path const &path)
{
	std::ostringstream oss;
	oss << "Change password '" << path.getRelativePath() << "'.";
	return oss.str();
}

struct WriteContext
{
	pwm::repository::Repository &repository;
	pwm::repository::Path const &path;
	std::ofstream out;

	WriteContext(pwm::repository::Repository &r,
	             pwm::repository::Path const &p)
	        : repository(r), path(p), out()
	{
		// Create the file's parent directory, if it doesn't already
		// exist.
		bdrck::fs::createPath(
		        bdrck::fs::dirname(path.getAbsolutePath()));

		// Open the password file for writing.
		out.open(path.getAbsolutePath(), std::ios_base::out |
		                                         std::ios_base::binary |
		                                         std::ios_base::trunc);
		if(!out.is_open())
		{
			throw std::runtime_error(
			        "Opening password file for writing failed.");
		}
	}

	~WriteContext()
	{
		out.close();

		// Commit the change.
		bdrck::git::Index index(*repository.repository);
		index.addAll({path.getRelativePath()});
		bdrck::git::commitIndex(*repository.repository,
		                        getPasswordChangeMessage(path));
	}
};
}

namespace pwm
{
namespace repository
{
std::string read(Repository const &, Path const &path)
{
	return bdrck::fs::readEntireFile(path.getAbsolutePath());
}

void write(Repository &repository, Path const &path, uint8_t const *data,
           std::size_t length)
{
	WriteContext context(repository, path);
	context.out.write(reinterpret_cast<char const *>(data),
	                  static_cast<std::streamsize>(length));
}

void write(Repository &repository, Path const &path, std::istream &in)
{
	WriteContext context(repository, path);
	std::istreambuf_iterator<char> inBegin(in);
	std::istreambuf_iterator<char> inEnd;
	std::ostreambuf_iterator<char> outBegin(context.out);
	std::copy(inBegin, inEnd, outBegin);
}
}
}
