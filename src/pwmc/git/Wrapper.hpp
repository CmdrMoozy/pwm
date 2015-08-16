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

#ifndef pwmc_git_Wrapper_HPP
#define pwmc_git_Wrapper_HPP

#include <memory>
#include <utility>

#include "pwmc/git/checkReturn.hpp"

namespace pwm
{
namespace git
{
template <typename T, typename... Arg>
T *constructGitObject(int (*f)(T **, Arg...), Arg... arg)
{
	T *o = nullptr;
	checkReturn(f(&o, std::forward<Arg...>(arg...)));
	return o;
}

template <typename T, void (*deleter)(T *)> class Wrapper
{
public:
	Wrapper(T *o) : object(o, deleter)
	{
	}

	template <typename... Arg>
	Wrapper(int (*f)(T **, Arg...), Arg... arg)
	        : Wrapper(constructGitObject(f, std::forward<Arg...>(arg...)))
	{
	}

	Wrapper(const Wrapper &) = delete;
	Wrapper(Wrapper &&) = default;

	virtual ~Wrapper() = default;

	Wrapper &operator=(const Wrapper &) = delete;
	Wrapper &operator=(Wrapper &&) = default;

	T *get() const
	{
		return object.get();
	}

private:
	std::unique_ptr<T, void (*)(T *)> object;
};
}
}

#endif
