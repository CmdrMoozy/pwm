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

#ifndef pwmc_util_Base64_HPP
#define pwmc_util_Base64_HPP

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace pwm
{
namespace util
{
std::string encodeBase64(void const* data, std::size_t length) noexcept;
std::vector<uint8_t> decodeBase64(std::string const& s);
}
}

#endif
