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

#include "deserializeConfiguration.hpp"

#include <cstddef>
#include <deque>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <utility>
#include <vector>

#include <yajl/yajl_parse.h>

#include "pwmc/util/ScopeExit.hpp"
#include "pwmc/util/String.hpp"

namespace
{
struct ParseState
{
	std::string error;

	pwm::config::ConfigurationData &data;
	std::deque<std::string> key;

	ParseState(pwm::config::ConfigurationData &d)
	        : error(""), data(d), key()
	{
	}
};

pwm::config::Key toKey(const std::deque<std::string> &components)
{
	return pwm::util::join(components.begin(), components.end(), ".");
}

int nullCallback(void *ctx)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->error = "Encountered invalid JSON value.";
	return 0;
}

int booleanCallback(void *ctx, int)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->error = "Encountered invalid JSON value.";
	return 0;
}

int numberCallback(void *ctx, const char *, std::size_t)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->error = "Encountered invalid JSON value.";
	return 0;
}

int stringCallback(void *ctx, const unsigned char *v, std::size_t l)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->data.data.insert(std::make_pair(
	        toKey(state->key),
	        std::string(reinterpret_cast<const char *>(v), l)));
	if(!state->key.empty()) state->key.pop_back();
	return 1;
}

int startMapCallback(void *)
{
	return 1;
}

int mapKeyCallback(void *ctx, const unsigned char *k, std::size_t l)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->key.emplace_back(reinterpret_cast<const char *>(k), l);
	return 1;
}

int endMapCallback(void *ctx)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	if(!state->key.empty()) state->key.pop_back();
	return 1;
}

int startArrayCallback(void *ctx)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->error = "Encountered invalid JSON value.";
	return 0;
}

int endArrayCallback(void *ctx)
{
	ParseState *state = static_cast<ParseState *>(ctx);
	state->error = "Encountered invalid JSON value.";
	return 0;
}

const yajl_callbacks CALLBACKS = {
        nullCallback, booleanCallback, nullptr, nullptr, numberCallback,
        stringCallback, startMapCallback, mapKeyCallback, endMapCallback,
        startArrayCallback, endArrayCallback};

const std::size_t BUFFER_SIZE = 4096;

void checkStatus(yajl_status status, const ParseState &state)
{
	if((status == yajl_status_client_canceled) && !state.error.empty())
	{
		std::ostringstream oss;
		oss << "JSON parsing failed: " << state.error;
		throw std::runtime_error(oss.str());
	}

	if(status != yajl_status_ok)
	{
		std::ostringstream oss;
		oss << "JSON parsing failed: "
		    << std::string(yajl_status_to_string(status));
		throw std::runtime_error(oss.str());
	}
}
}

namespace pwm
{
namespace config
{
ConfigurationData deserializeConfiguration(std::istream &in)
{
	ConfigurationData data;
	ParseState state(data);

	yajl_handle parser = yajl_alloc(&CALLBACKS, nullptr, &state);
	if(parser == nullptr)
		throw std::runtime_error("Creating JSON parser failed.");
	util::ScopeExit cleanup([&parser]()
	                        {
		                        yajl_free(parser);
		                });

	std::vector<char> buffer(BUFFER_SIZE, '\0');
	std::streamsize read;
	while((read = in.readsome(buffer.data(), BUFFER_SIZE)) > 0)
	{
		checkStatus(yajl_parse(parser,
		                       reinterpret_cast<const unsigned char *>(
		                               buffer.data()),
		                       static_cast<std::size_t>(read)),
		            state);
	}

	checkStatus(yajl_complete_parse(parser), state);

	return data;
}

ConfigurationData deserializeConfiguration(const std::string &p)
{
	std::ifstream in(p, std::ios_base::in | std::ios_base::binary);
	if(!in.is_open())
	{
		throw std::runtime_error(
		        "Opening configuration file for reading failed.");
	}
	return deserializeConfiguration(in);
}
}
}
