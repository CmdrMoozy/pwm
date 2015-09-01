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

#ifndef pwmc_util_MemoryFile_HPP
#define pwmc_util_MemoryFile_HPP

#include <cstddef>
#include <cstdint>
#include <memory>

namespace pwm
{
namespace util
{
class MemoryFile
{
public:
	MemoryFile();

	MemoryFile(MemoryFile const &) = delete;
	MemoryFile(MemoryFile &&) = default;
	MemoryFile &operator=(MemoryFile const &) = delete;
	MemoryFile &operator=(MemoryFile &&) = default;

	~MemoryFile();

	std::size_t write(uint8_t const *data, std::size_t length);
	void flush();

	std::size_t size() const;
	uint8_t const *data() const;

private:
	struct MemoryFileImpl;
	std::unique_ptr<MemoryFileImpl> impl;
};
}
}

#endif
