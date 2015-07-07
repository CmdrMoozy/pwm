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

#include "Clipboard.hpp"

#ifdef PWM_USE_CLIPBOARD
#include <gtk/gtk.h>
#endif

#ifdef PWM_USE_CLIPBOARD
namespace
{
GdkAtom clipboardTypeToAtom(pwm::util::clipboard::ClipboardType type)
{
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wold-style-cast"
	switch(type)
	{
	case pwm::util::clipboard::ClipboardType::Clipboard:
		return GDK_SELECTION_CLIPBOARD;
	case pwm::util::clipboard::ClipboardType::Primary:
		return GDK_SELECTION_PRIMARY;
	case pwm::util::clipboard::ClipboardType::Secondary:
		return GDK_SELECTION_SECONDARY;
	default:
		return GDK_NONE;
	}
#pragma GCC diagnostic pop
}
}
#endif

namespace pwm
{
namespace util
{
namespace clipboard
{
std::string getClipboardContents(ClipboardType
#ifdef PWM_USE_CLIPBOARD
                                         type
#endif
                                 )
{
#ifdef PWM_USE_CLIPBOARD
	GtkClipboard *clipboard = gtk_clipboard_get(clipboardTypeToAtom(type));
	gchar *text = gtk_clipboard_wait_for_text(clipboard);
	std::string ret(text);
	g_free(text);
	return ret;
#endif
}

void setClipboardContents(ClipboardType
#ifdef PWM_USE_CLIPBOARD
                                  type
#endif
                          ,
                          const std::string &
#ifdef PWM_USE_CLIPBOARD
                                  text
#endif
                          )
{
#ifdef PWM_USE_CLIPBOARD
	GtkClipboard *clipboard = gtk_clipboard_get(clipboardTypeToAtom(type));
	gtk_clipboard_set_text(clipboard, text.c_str(), text.length());
#endif
}
}
}
}
