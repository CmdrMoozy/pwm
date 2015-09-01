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

#include "MemoryFile.hpp"

#include <cstdlib>
#include <stdexcept>

namespace pwm
{
namespace util
{
struct MemoryFile::MemoryFileImpl
{
	FILE *stream;

	char *buffer;
	std::size_t size;

	MemoryFileImpl()
	{
		stream = open_memstream(&buffer, &size);
		if(stream == nullptr)
		{
			throw std::runtime_error("Opening memory file failed.");
		}
	}

	~MemoryFileImpl()
	{
		fclose(stream);
		std::free(buffer);
	}
};

MemoryFile::MemoryFile() : impl(new MemoryFileImpl())
{
}

MemoryFile::~MemoryFile()
{
}

std::size_t MemoryFile::write(uint8_t const *d, std::size_t length)
{
	return std::fwrite(d, sizeof(uint8_t), length, impl->stream);
}

void MemoryFile::flush()
{
	std::fflush(impl->stream);
}

std::size_t MemoryFile::size() const
{
	return impl->size;
}

uint8_t const *MemoryFile::data() const
{
	return reinterpret_cast<uint8_t const *>(impl->buffer);
}
}
}
