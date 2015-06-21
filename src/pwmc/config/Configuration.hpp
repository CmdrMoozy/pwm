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

#ifndef pwmc_config_Configuration_HPP
#define pwmc_config_Configuration_HPP

#include <memory>
#include <mutex>

namespace pwm
{
namespace config
{
class ConfigurationInstance
{
public:
	ConfigurationInstance();

	ConfigurationInstance(const ConfigurationInstance &) = delete;

	~ConfigurationInstance();

	ConfigurationInstance &operator=(const ConfigurationInstance &) = delete;
};

class Configuration
{
public:
	~Configuration() = default;

private:
	friend class ConfigurationInstance;

	static std::mutex mutex;
	static std::unique_ptr<Configuration> instance;

	Configuration();
};
}
}

#endif
