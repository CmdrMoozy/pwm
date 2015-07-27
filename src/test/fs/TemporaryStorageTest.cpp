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

#include <catch/catch.hpp>

#include "pwmc/fs/TemporaryStorage.hpp"
#include "pwmc/fs/Util.hpp"

TEST_CASE("Test temporary file behavior", "[FS]")
{
	std::string path;

	{
		pwm::fs::TemporaryStorage file(
		        pwm::fs::TemporaryStorageType::FILE);
		path = file.getPath();
		CHECK(pwm::fs::exists(path));
		CHECK(pwm::fs::isFile(path));
	}

	CHECK(!pwm::fs::isFile(path));
	CHECK(!pwm::fs::exists(path));
}

TEST_CASE("Test temporary directory behavior", "[FS]")
{
	std::string path;

	{
		pwm::fs::TemporaryStorage directory(
		        pwm::fs::TemporaryStorageType::DIRECTORY);
		path = directory.getPath();
		CHECK(pwm::fs::exists(path));
		CHECK(pwm::fs::isDirectory(path));

		// Add some random files and directories to the directory, to
		// make sure the removal still works if it is non-empty.

		std::string afile = pwm::fs::combinePaths(path, "a.txt");
		std::string bdir = pwm::fs::combinePaths(path, "b");
		std::string bdirafile = pwm::fs::combinePaths(bdir, "a.txt");
		std::string bdircdir = pwm::fs::combinePaths(bdir, "c");
		std::string bdircdirafile =
		        pwm::fs::combinePaths(bdircdir, "a.txt");

		CHECK_NOTHROW(pwm::fs::createFile(afile));
		CHECK_NOTHROW(pwm::fs::createDirectory(bdir));
		CHECK_NOTHROW(pwm::fs::createFile(bdirafile));
		CHECK_NOTHROW(pwm::fs::createDirectory(bdircdir));
		CHECK_NOTHROW(pwm::fs::createFile(bdircdirafile));
	}

	CHECK(!pwm::fs::isDirectory(path));
	CHECK(!pwm::fs::exists(path));
}
