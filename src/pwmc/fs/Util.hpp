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

#ifndef pwmc_fs_Util_HPP
#define pwmc_fs_Util_HPP

#include <string>

namespace pwm
{
namespace fs
{
std::string normalizePath(const std::string &p);

std::string combinePaths(const std::string &a, const std::string &b);

bool exists(const std::string &p);

bool isFile(std::string const &p);
bool isDirectory(std::string const &p);

void createFile(std::string const &p);
void removeFile(std::string const &p);
void createDirectory(std::string const &p);
void removeDirectory(std::string const &p);

void createPath(const std::string &p);

std::string getTemporaryDirectoryPath();
std::string getConfigurationFilePath();
}
}

#endif
