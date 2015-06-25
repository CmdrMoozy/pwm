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

#include "pwmc/config/deserializeConfiguration.hpp"
#include "pwmc/config/serializeConfiguration.hpp"
#include "pwmc/fs/Util.hpp"

namespace pwm
{
namespace config
{
const std::string USE_CONFIG_DEFAULT_VALUE("USE_CONFIGURATION");

std::mutex Configuration::mutex;
std::unique_ptr<Configuration> Configuration::instance;

ConfigurationData::ConfigurationData() : data()
{
}

ConfigurationData::ConfigurationData(const std::map<Key, std::string> &d)
        : data(d)
{
}

void ConfigurationData::apply(const ConfigurationData &o, bool overwrite)
{
	for(const auto &kv : o.data)
	{
		auto it = data.find(kv.first);
		if(!overwrite && (it != data.end())) continue;
		if(it == data.end())
			it = data.insert(kv).first;
		else
			it->second = kv.second;
	}
}
}
}

namespace
{
const pwm::config::ConfigurationData
        DEFAULT_CONFIG(std::map<pwm::config::Key, std::string>({}));
}

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

Configuration::~Configuration()
{
	try
	{
		serializeConfiguration(fs::getConfigurationFilePath(), data);
	}
	catch(...)
	{
	}
}

Configuration::Configuration()
        : data(deserializeConfiguration(fs::getConfigurationFilePath()))
{
	data.apply(DEFAULT_CONFIG);
}

std::ostream &operator<<(std::ostream &os, const ConfigurationData &d)
{
	for(const auto &kv : d.data)
		os << kv.first << " = " << kv.second << "\n";
	return os;
}
}
}
