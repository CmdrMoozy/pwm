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

#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <sstream>
#include <stdexcept>

#include <ftw.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "pwmc/util/String.hpp"

namespace
{
constexpr int FILE_TREE_WALK_OPEN_FDS = 128;

int removeDirectoryCallback(char const *p, struct stat const *, int t,
                            struct FTW *)
{
	switch(t)
	{
	case FTW_F:
	case FTW_SL:
	case FTW_SLN:
	{
		int ret = unlink(p);
		if(ret != 0)
			throw std::runtime_error(
			        "Removing directory contents failed.");
	}
	break;

	case FTW_D:
	case FTW_DP:
	{
		int ret = rmdir(p);
		if(ret != 0)
			throw std::runtime_error(
			        "Removing directory contents failed.");
	}
	break;

	case FTW_DNR:
	case FTW_NS:
	default:
		throw std::runtime_error("Removing directory contents failed.");
	}

	return FTW_CONTINUE;
}
}

namespace pwm
{
namespace fs
{
std::string normalizePath(const std::string &p)
{
	std::string ret = p;

	// Convert Windows-style separators to POSIX separators.
	std::transform(ret.begin(), ret.end(), ret.begin(),
	               [](const char &c) -> char
	               {
		               if(c == '\\') return '/';
		               return c;
		       });

	// Remove any trailing separators.
	while(!ret.empty() && (*ret.rbegin() == '/'))
		ret.erase(ret.length() - 1);

	return ret;
}

std::string combinePaths(const std::string &a, const std::string &b)
{
	auto aEnd = a.find_last_not_of("\\/");
	auto bStart = b.find_first_not_of("\\/");

	std::ostringstream oss;
	if(aEnd != std::string::npos)
	{
		oss << a.substr(0, aEnd + 1);
	}
	else
	{
		// a must have been "/" (or an empty string). Prepend the root
		// directory to b to make a valid final path.
		oss << "/";
	}
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

bool isFile(std::string const &p)
{
	struct stat stats;
	int ret = stat(p.c_str(), &stats);
	if(ret != 0) return false;
	return S_ISREG(stats.st_mode);
}

bool isDirectory(std::string const &p)
{
	struct stat stats;
	int ret = stat(p.c_str(), &stats);
	if(ret != 0) return false;
	return S_ISDIR(stats.st_mode);
}

void createFile(std::string const &p)
{
	FILE *f = fopen(p.c_str(), "a");
	if(f == nullptr) throw std::runtime_error("Creating file failed.");
	fclose(f);
}

void removeFile(std::string const &p)
{
	if(!exists(p)) return;
	if(!isFile(p))
		throw std::runtime_error(
		        "Cannot remove non-file paths with this function.");
	int ret = std::remove(p.c_str());
	if(ret != 0) throw std::runtime_error("Removing file failed.");
}

void createDirectory(std::string const &p)
{
	if(isDirectory(p)) return;
	int ret = mkdir(p.c_str(), 0777);
	if(ret != 0) throw std::runtime_error("Creating directory failed.");
}

void removeDirectory(std::string const &p)
{
	if(!exists(p)) return;
	if(!isDirectory(p))
		throw std::runtime_error("Cannot remove non-directory paths "
		                         "with this function.");

	int ret = nftw(p.c_str(), removeDirectoryCallback,
	               FILE_TREE_WALK_OPEN_FDS,
	               FTW_ACTIONRETVAL | FTW_DEPTH | FTW_PHYS);
	if(ret != 0)
	{
		throw std::runtime_error("Removing directory contents failed.");
	}
}

void createPath(const std::string &p)
{
	std::vector<std::string> components =
	        pwm::util::split(normalizePath(p), '/');
	std::string currentPath = "";

	for(const auto &component : components)
	{
		currentPath = combinePaths(currentPath, component);
		if(isDirectory(currentPath)) continue;
		if(!exists(currentPath)) createDirectory(currentPath);
	}
}

std::string getTemporaryDirectoryPath()
{
	std::string path("/tmp");

	char const *tmpdir = std::getenv("TMPDIR");
	if(tmpdir != nullptr)
	{
		std::string tmpdirStr(tmpdir);
		if(isDirectory(tmpdirStr)) path = tmpdirStr;
	}

	return path;
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

	if(!exists(path)) createDirectory(path);

	if(!isDirectory(path))
	{
		throw std::runtime_error(
		        "Configuration directory is not a directory.");
	}

	return combinePaths(path, "pwm.conf");
}
}
}
