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

#include "Configuration.hpp"

#include <stdexcept>

std::mutex pwm::config::Configuration::mutex;
std::unique_ptr<pwm::config::Configuration>
        pwm::config::Configuration::instance;

namespace pwm
{
namespace config
{
ConfigurationInstance::ConfigurationInstance()
{
	std::lock_guard<std::mutex> lock(Configuration::mutex);
	if(!!Configuration::instance)
	{
		throw std::runtime_error(
		        "Can't initialize two Configuration instances.");
	}
	Configuration::instance.reset(new Configuration());
}

ConfigurationInstance::~ConfigurationInstance()
{
	std::lock_guard<std::mutex> lock(Configuration::mutex);
	if(!Configuration::instance)
	{
		throw std::runtime_error(
		        "No Configuration instance initialized.");
	}
	Configuration::instance.reset();
}

Configuration::Configuration()
{
}
}
}
