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

#ifndef pwmc_util_String_HPP
#define pwmc_util_String_HPP

#include <algorithm>
#include <functional>
#include <locale>
#include <sstream>
#include <string>
#include <vector>

namespace pwm
{
namespace util
{
std::string toLower(const std::string &s);

std::vector<std::string> split(const std::string &s, char d);

template <typename Iterator>
std::string join(Iterator begin, Iterator end, const std::string &delimiter)
{
	std::ostringstream oss;
	for(auto it = begin; it != end; ++it)
	{
		oss << *it;

		auto next = it;
		++next;
		if(next != end) oss << delimiter;
	}
	return oss.str();
}

template <typename UnaryPredicate = std::function<bool(char const &)>>
std::string &leftTrim(std::string &s,
                      UnaryPredicate predicate = [](char const &c) -> bool
                                                 {
	                                                 std::locale locale;
	                                                 return std::isspace(
	                                                         c, locale);
	                                         })
{
	auto it = std::find_if(s.begin(), s.end(), [&predicate](char const &c)
	                                                   -> bool
	                                           {
		                                           return !predicate(c);
		                                   });
	s.erase(s.begin(), it);
	return s;
}

template <typename UnaryPredicate = std::function<bool(char const &)>>
std::string &rightTrim(std::string &s,
                       UnaryPredicate predicate = [](char const &c) -> bool
                                                  {
	                                                  std::locale locale;
	                                                  return std::isspace(
	                                                          c, locale);
	                                          })
{
	auto it = std::find_if(s.rbegin(), s.rend(),
	                       [&predicate](char const &c) -> bool
	                       {
		                       return !predicate(c);
		               });
	s.erase(it.base(), s.end());
	return s;
}

template <typename UnaryPredicate = std::function<bool(char const &)>>
std::string &trim(std::string &s,
                  UnaryPredicate predicate = [](char const &c) -> bool
                                             {
	                                             std::locale locale;
	                                             return std::isspace(
	                                                     c, locale);
	                                     })
{
	return rightTrim(leftTrim(s, predicate), predicate);
}

std::string &removeRepeatedCharacters(std::string &str, char character);
}
}

#endif
