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

#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <string>

#include "pwmc/config/Key.hpp"

namespace pwm
{
namespace config
{
enum class ConfigurationValue
{
	RepositoryDefaultPath
};

Key getConfigurationKey(ConfigurationValue value);

struct ConfigurationData
{
	using ConfigurationMap = std::map<Key, std::string>;
	using const_iterator = ConfigurationMap::const_iterator;

	std::map<Key, std::string> data;

	ConfigurationData();
	explicit ConfigurationData(const std::map<Key, std::string> &d);

	ConfigurationData(const ConfigurationData &) = default;
	~ConfigurationData() = default;
	ConfigurationData &operator=(const ConfigurationData &) = default;

	void apply(const ConfigurationData &o, bool overwrite = false);
};

class ConfigurationInstance
{
public:
	ConfigurationInstance();

	ConfigurationInstance(const ConfigurationInstance &) = delete;

	~ConfigurationInstance();

	ConfigurationInstance &
	operator=(const ConfigurationInstance &) = delete;
};

class Configuration
{
public:
	static Configuration &getInstance();

	~Configuration();

	ConfigurationData::const_iterator begin() const;
	ConfigurationData::const_iterator end() const;

	std::string get(const Key &key) const;
	std::string getOr(const Key &key, const std::string &defaultVal) const;
	void set(const Key &key, const std::string &value);
	void reset(const Key &key);

private:
	friend class ConfigurationInstance;

	static std::mutex mutex;
	static std::unique_ptr<Configuration> instance;

	ConfigurationData data;

	Configuration();
};

std::ostream &operator<<(std::ostream &os, const ConfigurationData &d);
}
}

#endif
