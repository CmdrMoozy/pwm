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

#include "Library.hpp"

#include <stdexcept>

#include <git2.h>

#include "pwmc/git/checkReturn.hpp"

std::mutex pwm::git::Library::mutex;
std::unique_ptr<pwm::git::Library> pwm::git::Library::instance;

namespace pwm
{
namespace git
{
LibraryInstance::LibraryInstance()
{
	std::lock_guard<std::mutex> lock(Library::mutex);
	if(!!Library::instance)
		throw std::runtime_error("Can't initialize libgit2 twice.");
	Library::instance.reset(new Library());
}

LibraryInstance::~LibraryInstance()
{
	std::lock_guard<std::mutex> lock(Library::mutex);
	if(!Library::instance)
		throw std::runtime_error("libgit2 not initialized yet.");
	Library::instance.reset();
}

bool Library::isInitialized()
{
	std::lock_guard<std::mutex> lock(mutex);
	return !!instance;
}

Library::~Library()
{
	git_libgit2_shutdown();
}

Library::Library()
{
	checkReturn(git_libgit2_init());
}
}
}
