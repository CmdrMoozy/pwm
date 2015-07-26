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

#include "Option.hpp"

#include <algorithm>
#include <set>
#include <stdexcept>
#include <utility>

namespace
{
struct OptionNameComparator
{
	bool operator()(std::shared_ptr<pwm::params::Option> a,
	                std::shared_ptr<pwm::params::Option> b)
	{
		return a->name < b->name;
	}
};

struct OptionShortNameComparator
{
	bool operator()(std::shared_ptr<pwm::params::Option> a,
	                std::shared_ptr<pwm::params::Option> b)
	{
		return a->shortName < b->shortName;
	}
};
}

namespace pwm
{
namespace params
{
Option Option::required(std::string const &n, std::string const &h,
                        std::experimental::optional<char> const &sn,
                        std::experimental::optional<std::string> const &dv)
{
	return Option(n, h, sn, dv, false);
}

Option Option::required(std::string const &n, std::string const &h,
                        std::experimental::optional<char> const &sn,
                        std::string const &dv)
{
	return Option(n, h, sn, dv, false);
}

Option Option::flag(std::string const &n, std::string const &h,
                    std::experimental::optional<char> const &sn)
{
	return Option(n, h, sn, std::experimental::nullopt, true);
}

Option::Option(std::string const &n, std::string const &h,
               std::experimental::optional<char> const &sn,
               std::experimental::optional<std::string> const &dv, bool f)
        : name(n), help(h), shortName(sn), defaultValue(dv), isFlag(f)
{
}

Option::Option(std::string const &n)
        : Option(n, n, std::experimental::nullopt, std::experimental::nullopt,
                 false)
{
}

OptionSetConstIterator::OptionSetConstIterator()
        : data(nullptr), length(0), current(0)
{
}

Option const &OptionSetConstIterator::operator*() const
{
	return *data[current];
}

OptionSetConstIterator &OptionSetConstIterator::operator++()
{
	current = std::min(current + 1, length);
	if(current == length)
	{
		data = nullptr;
		length = 0;
		current = 0;
	}
	return *this;
}

bool OptionSetConstIterator::operator==(OptionSetConstIterator const &o)
{
	return (data == o.data) && (length == o.length) &&
	       (current == o.current);
}

bool OptionSetConstIterator::operator!=(OptionSetConstIterator const &o)
{
	return !(*this == o);
}

OptionSetConstIterator::OptionSetConstIterator(
        std::vector<std::shared_ptr<Option>> const &v)
        : data(v.data()), length(v.size()), current(0)
{
}

struct OptionSet::OptionSetImpl
{
	std::vector<std::shared_ptr<Option>> unorderedOptions;
	std::set<std::shared_ptr<Option>, OptionNameComparator> nameOptions;
	std::set<std::shared_ptr<Option>, OptionShortNameComparator>
	        shortNameOptions;

	OptionSetImpl(std::initializer_list<Option> const &options)
	{
		for(auto const &option : options)
		{
			auto optionPtr = std::make_shared<Option>(option);
			unorderedOptions.push_back(optionPtr);
			nameOptions.insert(optionPtr);
			if(!!option.shortName)
				shortNameOptions.insert(optionPtr);
		}
	}

	OptionSetImpl(OptionSetImpl const &) = default;
	OptionSetImpl(OptionSetImpl &&) = default;
	OptionSetImpl &operator=(OptionSetImpl const &) = default;
	OptionSetImpl &operator=(OptionSetImpl &&) = default;

	~OptionSetImpl() = default;
};

OptionSet::OptionSet(std::initializer_list<Option> const &o)
        : impl(new OptionSetImpl(o))
{
}

OptionSet::OptionSet(OptionSet const &o)
{
	*this = o;
}

OptionSet::OptionSet(OptionSet &&o)
{
	impl = std::move(o.impl);
}

OptionSet &OptionSet::operator=(OptionSet const &o)
{
	if(this == &o) return *this;
	if(o.impl)
		impl.reset(new OptionSetImpl(*o.impl));
	else
		impl.reset();
	return *this;
}

OptionSet &OptionSet::operator=(OptionSet &&o)
{
	impl = std::move(o.impl);
	return *this;
}

OptionSet::~OptionSet()
{
}

std::size_t OptionSet::size() const
{
	return impl->unorderedOptions.size();
}

OptionSetConstIterator OptionSet::begin() const
{
	return OptionSetConstIterator(impl->unorderedOptions);
}

OptionSetConstIterator OptionSet::end() const
{
	return OptionSetConstIterator();
}

Option const *OptionSet::find(std::string const &parameter) const
{
	std::shared_ptr<Option> search(new Option(parameter));
	if(parameter.length() == 1) search->shortName = parameter[0];

	auto nameIt = impl->nameOptions.find(search);
	if(nameIt != impl->nameOptions.end()) return &(**nameIt);

	if(!!search->shortName)
	{
		auto shortNameIt = impl->shortNameOptions.find(search);
		if(shortNameIt != impl->shortNameOptions.end())
			return &(**shortNameIt);
	}

	return nullptr;
}
}
}
