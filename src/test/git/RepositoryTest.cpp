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

#include <bdrck/fs/TemporaryStorage.hpp>
#include <bdrck/fs/Util.hpp>

#include "pwmc/git/Repository.hpp"

TEST_CASE("Test Git repository work directory path retrieval", "[Git]")
{
	bdrck::fs::TemporaryStorage directory(
	        bdrck::fs::TemporaryStorageType::DIRECTORY);
	REQUIRE(bdrck::fs::isDirectory(directory.getPath()));
	pwm::git::Repository repository(directory.getPath());
	CHECK(bdrck::fs::normalizePath(directory.getPath()) ==
	      bdrck::fs::normalizePath(repository.getWorkDirectoryPath()));
}
