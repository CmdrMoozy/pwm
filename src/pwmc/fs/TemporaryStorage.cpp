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

#include "TemporaryStorage.hpp"

#include <cassert>
#include <sstream>
#include <stdexcept>

#include "pwmc/fs/Util.hpp"
#include "pwmc/util/UUID.hpp"

namespace
{
std::string getTemporaryPath()
{
	std::ostringstream oss;
	oss << "pwm-" << pwm::util::generateUUID() << ".tmp";
	return pwm::fs::combinePaths(pwm::fs::getTemporaryDirectoryPath(),
	                             oss.str());
}
}

namespace pwm
{
namespace fs
{
TemporaryStorage::TemporaryStorage(TemporaryStorageType t)
        : type(t), path(getTemporaryPath())
{
	while(exists(path))
		path = getTemporaryPath();

	switch(type)
	{
	case TemporaryStorageType::FILE:
		createFile(path);
		if(!isFile(path))
		{
			throw std::runtime_error(
			        "Creating temporary file failed.");
		}
		break;

	case TemporaryStorageType::DIRECTORY:
		createDirectory(path);
		if(!isDirectory(path))
		{
			throw std::runtime_error(
			        "Creating temporary directory failed.");
		}
		break;

	default:
		throw std::runtime_error("Unsupported temporary storage type.");
	}
}

TemporaryStorage::~TemporaryStorage()
{
	switch(type)
	{
	case TemporaryStorageType::FILE:
		removeFile(path);
		break;

	case TemporaryStorageType::DIRECTORY:
		removeDirectory(path);
		break;

	default:
		assert(false);
	}
}

std::string TemporaryStorage::getPath() const
{
	return path;
}
}
}
