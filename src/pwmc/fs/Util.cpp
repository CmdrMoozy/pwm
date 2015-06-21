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

#include "Util.hpp"

#include <cstdlib>
#include <sstream>
#include <stdexcept>

#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>

namespace pwm
{
namespace fs
{
std::string combinePaths(const std::string &a, const std::string &b)
{
	auto aEnd = a.find_last_not_of("\\/");
	auto bStart = b.find_first_not_of("\\/");

	std::ostringstream oss;
	if(aEnd != std::string::npos) oss << a.substr(0, aEnd + 1);
	if((aEnd != std::string::npos) && (bStart != std::string::npos))
		oss << "/";
	if(bStart != std::string::npos) oss << b.substr(bStart);

	return oss.str();
}

bool exists(const std::string &p)
{
	struct stat stats;
	int ret = stat(p.c_str(), &stats);
	return ret == 0;
}

bool isDirectory(const std::string &p)
{
	struct stat stats;
	int ret = stat(p.c_str(), &stats);
	if(ret != 0) return false;
	return S_ISDIR(stats.st_mode);
}

std::string getConfigurationFilePath()
{
	std::string path;
	std::string suffix;

	char *home = getenv("XDG_CONFIG_HOME");
	if(home == nullptr)
	{
		home = getenv("HOME");
		if(home == nullptr)
		{
			throw std::runtime_error(
			        "Couldn't find home directory.");
		}
		suffix.assign(".config");
	}
	path.assign(home);
	path = combinePaths(path, suffix);

	if(!exists(path))
	{
		int ret = mkdir(path.c_str(), 0777);
		if(ret != 0)
		{
			throw std::runtime_error(
			        "Creating configuration directory failed.");
		}
	}

	if(!isDirectory(path))
	{
		throw std::runtime_error(
		        "Configuration directory is not a directory.");
	}

	return combinePaths(path, "pwm.conf");
}
}
}
