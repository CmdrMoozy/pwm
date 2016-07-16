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

#include "passwordPrompt.hpp"

#include <cstdio>
#include <iostream>

#include <boost/optional/optional.hpp>

#include <termios.h>
#include <unistd.h>

#include <bdrck/util/Error.hpp>
#include <bdrck/util/ScopeExit.hpp>

namespace
{
std::string singlePasswordPrompt(std::string const &prompt)
{
	std::cout << prompt;
	boost::optional<std::string> password;

	{
		termios oldt;
		int ret = tcgetattr(STDIN_FILENO, &oldt);
		if(ret != 0) bdrck::util::error::throwErrnoError();
		bdrck::util::ScopeExit cleanup(
		        [&oldt]() { tcsetattr(STDIN_FILENO, TCSANOW, &oldt); });

		termios newt{oldt};
		newt.c_lflag &= ~static_cast<tcflag_t>(ECHO);
		ret = tcsetattr(STDIN_FILENO, TCSANOW, &newt);
		if(ret != 0) bdrck::util::error::throwErrnoError();

		std::string p;
		std::getline(std::cin, p);
		password.emplace(std::move(p));
	}

	std::cout << "\n";
	return *password;
}
}

namespace pwm
{
namespace util
{
std::string passwordPrompt(bool confirm)
{
	boost::optional<std::string> password;
	while(!password)
	{
		password.emplace(singlePasswordPrompt("Password: "));
		if(confirm)
		{
			std::string confirmPassword{
			        singlePasswordPrompt("Confirm: ")};
			if(*password != confirmPassword) password = boost::none;
		}
	}
	return *password;
}
}
}
