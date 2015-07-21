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

#include <catch/catch.hpp>

#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

#include "pwmc/util/Clipboard.hpp"

namespace
{
const std::string INITIAL_CLIPBOARD_CONTENTS = "";
const std::string TEST_CLIPBOARD_CONTENTS = "test";
}

TEST_CASE("Test clipboard persistence after process exit", "[Clipboard]")
{
#ifdef PWM_USE_CLIPBOARD
/*
// We want to fork a new process, and set the clipboard contents using
// that process. We should be able to see the new clipboard contents
// even after that other process has exited.

// Ensure the existing clipboard content isn't our test string.
pwm::util::clipboard::setClipboardContents(
        pwm::util::clipboard::ClipboardType::Clipboard,
        INITIAL_CLIPBOARD_CONTENTS);

pid_t pid = fork();
REQUIRE(pid >= 0);
if(pid == 0)
{
        // Child process. Set the clipboard contents.
        pwm::util::clipboard::setClipboardContents(
                pwm::util::clipboard::ClipboardType::Clipboard,
                TEST_CLIPBOARD_CONTENTS);
        _exit(0);
}

// Wait on the child process to exit, and then test the clipboard.
int status;
pid_t wpid = waitpid(pid, &status, 0);
REQUIRE(wpid == pid);
REQUIRE(TEST_CLIPBOARD_CONTENTS ==
        pwm::util::clipboard::getClipboardContents(
                pwm::util::clipboard::ClipboardType::Clipboard));
*/
#endif
}
