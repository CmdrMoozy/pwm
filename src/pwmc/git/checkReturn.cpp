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

#include "checkReturn.hpp"

#include <stdexcept>
#include <string>

#include <git2.h>

namespace pwm
{
namespace git
{
void checkReturn(int r)
{
	if(r == 0) return;
	const git_error *err = giterr_last();
	if(err == nullptr)
		return;
	std::string errMsg(err->message);
	giterr_clear();
	throw std::runtime_error(errMsg);
}
}
}
