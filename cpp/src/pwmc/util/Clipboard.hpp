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

#ifndef pwmc_util_Clipboard_HPP
#define pwmc_util_Clipboard_HPP

#include <string>

namespace pwm
{
namespace util
{
namespace clipboard
{
enum class ClipboardType
{
	Clipboard, // Typical cross-platform clipboard.
	Primary,   // Used by X11 for selected but not copied text.
	Secondary  // Generally unused.
};

std::string getClipboardContents(ClipboardType type);
void setClipboardContents(ClipboardType type, const std::string &text);
}
}
}

#endif