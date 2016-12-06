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
#include <vector>

#include <bdrck/fs/Util.hpp>
#include <bdrck/git/Commit.hpp>
#include <bdrck/git/Index.hpp>
#include <bdrck/git/StrArray.hpp>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/decrypt.hpp"
#include "pwmc/crypto/encrypt.hpp"
#include "pwmc/repository/buildMasterKey.hpp"

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

	void write(uint8_t const *plaintext, std::size_t size)
	{
		auto data = pwm::crypto::encrypt(
		        pwm::repository::buildMasterKey(repository), plaintext,
		        size);
		out.write(reinterpret_cast<char const *>(data.data()),
		          static_cast<std::streamsize>(data.size()));
	}
};
}

namespace pwm
{
namespace repository
{
std::string read(Repository const &repository, Path const &path)
{
	std::ifstream in(path.getAbsolutePath());
	if(!in.is_open())
	{
		std::ostringstream oss;
		oss << "Failed to open password file '"
		    << path.getRelativePath() << "' for reading.";
		throw std::runtime_error(oss.str());
	}

	std::vector<char> ciphertext;
	std::istreambuf_iterator<char> inBegin(in);
	std::istreambuf_iterator<char> inEnd;
	std::copy(inBegin, inEnd, std::back_inserter(ciphertext));
	std::vector<uint8_t> plaintext = pwm::crypto::decrypt(
	        pwm::repository::buildMasterKey(repository),
	        reinterpret_cast<uint8_t const *>(ciphertext.data()),
	        ciphertext.size());
	return std::string(reinterpret_cast<char const *>(plaintext.data()),
	                   plaintext.size());
}

void write(Repository &repository, Path const &path, uint8_t const *data,
           std::size_t length)
{
	WriteContext context(repository, path);
	context.write(data, length);
}

void write(Repository &repository, Path const &path, std::istream &in)
{
	WriteContext context(repository, path);
	std::vector<char> plaintext;
	std::istreambuf_iterator<char> inBegin(in);
	std::istreambuf_iterator<char> inEnd;
	std::copy(inBegin, inEnd, std::back_inserter(plaintext));
	context.write(reinterpret_cast<uint8_t const *>(plaintext.data()),
	              plaintext.size());
}
}
}
