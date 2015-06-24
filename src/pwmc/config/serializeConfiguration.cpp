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

#include "serializeConfiguration.hpp"

#include <algorithm>
#include <cstddef>
#include <map>
#include <sstream>
#include <stdexcept>
#include <utility>

#include <yajl/yajl_gen.h>

#include "pwmc/config/ConfigurationRadixTree.hpp"
#include "pwmc/util/ScopeExit.hpp"

namespace
{
void checkStatus(yajl_gen_status status)
{
	std::ostringstream oss;
	oss << "JSON generation error: ";

	switch(status)
	{
	case yajl_gen_status_ok:
	case yajl_gen_generation_complete:
		return;
	case yajl_gen_keys_must_be_strings:
		oss << "keys must be strings.";
		break;
	case yajl_max_depth_exceeded:
		oss << "maximum generation depth exceeded.";
		break;
	case yajl_gen_in_error_state:
		oss << "generation function called while generator was in an "
		       "error state.";
		break;
	case yajl_gen_invalid_number:
		oss << "invalid number (infinity or NaN) encountered.";
		break;
	case yajl_gen_no_buf:
		oss << "missing internal buffer.";
		break;
	case yajl_gen_invalid_string:
		oss << "invalid UTF8 string contents.";
		break;
	default:
		oss << "unknown error.";
		break;
	}

	throw std::runtime_error(oss.str());
}

template <typename... Arg>
void generate(yajl_gen_status (*f)(yajl_gen, Arg...), yajl_gen gen, Arg... arg)
{
	checkStatus(f(gen, std::forward<Arg>(arg)...));
}

bool preTraverse(yajl_gen gen, const std::string &key, const std::string &value)
{
	const unsigned char *keyStr =
	        reinterpret_cast<const unsigned char *>(key.data());
	generate(yajl_gen_string, gen, keyStr, key.length());

	if(value.empty())
	{
		generate(yajl_gen_map_open, gen);
	}
	else
	{
		const unsigned char *valStr =
		        reinterpret_cast<const unsigned char *>(value.data());
		generate(yajl_gen_string, gen, valStr, value.length());
	}

	return true;
}

bool postTraverse(yajl_gen gen, const std::string &, const std::string &value)
{
	if(value.empty()) generate(yajl_gen_map_close, gen);

	return true;
}
}

namespace pwm
{
namespace config
{
std::string serializeConfiguration(const ConfigurationData &data,
                                   bool formatted)
{
	yajl_gen gen = yajl_gen_alloc(nullptr);
	if(gen == nullptr)
		throw std::runtime_error("Creating JSON generator failed.");
	util::ScopeExit cleanup([&gen]()
	                        {
		                        yajl_gen_free(gen);
		                });

	yajl_gen_config(gen, yajl_gen_validate_utf8, 1);
	if(formatted) yajl_gen_config(gen, yajl_gen_beautify, 1);

	ConfigurationRadixTree tree(data);

	generate(yajl_gen_map_open, gen);
	tree.traverse(
	        [gen](const std::string &k, const std::string &v) -> bool
	        {
		        return preTraverse(gen, k, v);
		},
	        [gen](const std::string &k, const std::string &v) -> bool
	        {
		        return postTraverse(gen, k, v);
		});
	generate(yajl_gen_map_close, gen);

	const unsigned char *buf;
	std::size_t length;
	checkStatus(yajl_gen_get_buf(gen, &buf, &length));
	const char *strBuf = reinterpret_cast<const char *>(buf);

	return std::string(strBuf, length);
}
}
}
