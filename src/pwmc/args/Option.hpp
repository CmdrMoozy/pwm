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

#ifndef pwmc_args_Option_HPP
#define pwmc_args_Option_HPP

#include <string>

namespace pwm
{
namespace args
{
struct Option
{
	std::string name;
	std::string help;
	char shortName;
	std::string defaultVal;
	bool flag;

	Option(const std::string &n, const std::string& h,
		char sn = '\0', const std::string& dv = "", bool f = false);

	Option(const Option &) = default;
	~Option() = default;
	Option &operator=(const Option &) = default;
};
}
}

#endif
