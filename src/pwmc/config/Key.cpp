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

#include "Key.hpp"

#include <algorithm>
#include <locale>
#include <stdexcept>

#include "pwmc/util/String.hpp"

namespace pwm
{
namespace config
{
Key::Key(const std::string &k) : components(util::split(k, '.'))
{
	if(components.size() == 0)
		throw std::runtime_error("Configuration key must be nonempty.");

	std::locale locale;
	for(const auto &component : components)
	{
		auto it =
		        std::find_if_not(component.begin(), component.end(),
		                         [&locale](const char &c)
		                         {
			                         return std::isalpha(c, locale);
			                 });
		if(it != component.end())
		{
			throw std::runtime_error("Configuration keys can only "
			                         "contain alphabetic "
			                         "characters.");
		}
	}
}

bool operator<(const Key &a, const Key &b)
{
	return a.components < b.components;
}

bool operator==(const Key &a, const Key &b)
{
	return a.components == b.components;
}

std::ostream &operator<<(std::ostream &os, const Key &k)
{
	os << pwm::util::join(k.components.begin(), k.components.end(), ".");
	return os;
}
}
}
