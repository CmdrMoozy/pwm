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

#ifndef pwmc_params_Option_HPP
#define pwmc_params_Option_HPP

#include <string>
#include <experimental/optional>

namespace pwm
{
namespace params
{
struct Option
{
	std::string name;
	std::string help;
	std::experimental::optional<char> shortName;
	std::experimental::optional<std::string> defaultValue;
	bool flag;

	Option(std::string const& n,
		std::string const& h,
		std::experimental::optional<char> const& sn = std::experimental::nullopt,
		std::experimental::optional<std::string> const& dv = std::experimental::nullopt,
		bool f = false);

	Option(std::string const& n,
		std::string const& h,
		std::experimental::optional<char> const& sn,
		std::string const& dv);
};
}
}

#endif
