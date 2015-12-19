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

#include <cstddef>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <string>
#include <vector>
#include <experimental/optional>

namespace pwm
{
namespace params
{
class OptionSet;

struct Option
{
public:
	static Option required(std::string const &n, std::string const &h,
	                       std::experimental::optional<char> const &sn =
	                               std::experimental::nullopt,
	                       std::experimental::optional<std::string> const
	                               &dv = std::experimental::nullopt);

	static Option required(std::string const &n, std::string const &h,
	                       std::experimental::optional<char> const &sn,
	                       std::string const &dv);

	static Option optional(std::string const &n, std::string const &h,
	                       std::experimental::optional<char> const &sn =
	                               std::experimental::nullopt);

	static Option flag(std::string const &n, std::string const &h,
	                   std::experimental::optional<char> const &sn =
	                           std::experimental::nullopt);

	std::string name;
	std::string help;
	std::experimental::optional<char> shortName;
	std::experimental::optional<std::string> defaultValue;
	bool isOptional;
	bool isFlag;

private:
	friend class OptionSet;

	Option(std::string const &n, std::string const &h,
	       std::experimental::optional<char> const &sn,
	       std::experimental::optional<std::string> const &dv, bool o,
	       bool f);

	Option(std::string const &n);
};

class OptionSetConstIterator
{
public:
	typedef std::size_t difference_type;
	typedef Option value_type;
	typedef Option const &reference;
	typedef Option const *pointer;
	typedef std::forward_iterator_tag iterator_category;

	OptionSetConstIterator();

	OptionSetConstIterator(OptionSetConstIterator const &) = default;
	OptionSetConstIterator &
	operator=(OptionSetConstIterator const &) = default;

	~OptionSetConstIterator() = default;

	Option const &operator*() const;
	OptionSetConstIterator &operator++();

	bool operator==(OptionSetConstIterator const &o);
	bool operator!=(OptionSetConstIterator const &o);

private:
	friend class OptionSet;

	OptionSetConstIterator(std::vector<std::shared_ptr<Option>> const &v);

	std::shared_ptr<Option> const *data;
	std::size_t length;
	std::size_t current;
};

class OptionSet
{
private:
	struct OptionSetImpl;

public:
	OptionSet(std::initializer_list<Option> const &o);

	OptionSet(OptionSet const &o);
	OptionSet(OptionSet &&o);
	OptionSet &operator=(OptionSet const &o);
	OptionSet &operator=(OptionSet &&o);

	~OptionSet();

	std::size_t size() const;

	OptionSetConstIterator begin() const;
	OptionSetConstIterator end() const;

	Option const *find(std::string const &parameter) const;

private:
	std::unique_ptr<OptionSetImpl> impl;
};
}
}

#endif
