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

#include <mutex>

#include <bdrck/config/Util.hpp>

namespace
{
constexpr char const *CONFIG_ID_APPLICATION = "pwm";
constexpr char const *CONFIG_ID_NAME = "configuration";

bdrck::config::ConfigurationIdentifier getConfigurationIdentifier()
{
	static const bdrck::config::ConfigurationIdentifier identifier{CONFIG_ID_APPLICATION, CONFIG_ID_NAME};
	return identifier;
}

pwm::proto::Configuration getDefaultConfiguration()
{
	static std::mutex mutex;
	static bool initialized{false};
	static pwm::proto::Configuration defaults;

	std::lock_guard<std::mutex> lock(mutex);
	if(!initialized)
	{
		initialized = true;
	}

	return defaults;
}
}

namespace pwm
{
namespace config
{
ConfigurationInstance::ConfigurationInstance(boost::optional<std::string> const &customPath)
	: instanceHandle(getConfigurationIdentifier(), getDefaultConfiguration(), customPath)
{
}

bdrck::config::Configuration<pwm::proto::Configuration>& instance()
{
	return bdrck::config::Configuration<pwm::proto::Configuration>::instance(getConfigurationIdentifier());
}

std::string getFieldAsString(std::string const& path)
{
	return bdrck::config::getFieldAsString(path, instance().get());
}

void setFieldFromString(std::string const& path, std::string const& value)
{
	auto message = instance().get();
	bdrck::config::setFieldFromString(path, message, value);
	instance().set(message);
}
}
}
