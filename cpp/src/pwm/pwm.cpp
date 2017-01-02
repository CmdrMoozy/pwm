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

#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <initializer_list>
#include <iostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include <boost/optional/optional.hpp>

#ifdef PWM_USE_CLIPBOARD
#include <gtk/gtk.h>
#endif

#include <bdrck/config/Configuration.hpp>
#include <bdrck/git/Library.hpp>
#include <bdrck/git/Repository.hpp>
#include <bdrck/params/Argument.hpp>
#include <bdrck/params/Command.hpp>
#include <bdrck/params/Option.hpp>
#include <bdrck/params/parseAndExecuteCommand.hpp>

#include "pwmc/config/Configuration.hpp"
#include "pwmc/repository/IO.hpp"
#include "pwmc/repository/Path.hpp"
#include "pwmc/repository/Repository.hpp"
#include "pwmc/repository/list.hpp"
#include "pwmc/util/passwordPrompt.hpp"

#ifdef PWM_DEBUG
#include "pwmc/util/Clipboard.hpp"
#endif

namespace
{
std::string getRepositoryPath(bdrck::params::OptionsMap const &options)
{
	std::string repoPath =
	        pwm::config::instance().get().default_repository();
	auto repoIt = options.find("repository");
	if(repoIt != options.end()) repoPath = repoIt->second;

	if(repoPath.empty())
	{
		std::ostringstream oss;
		oss << "No repository path specified. Try the 'repository' "
		       "command option, or setting the 'default_repository' "
		       "configuration key.";
		throw std::runtime_error(oss.str());
	}

	return repoPath;
}

void configCommand(bdrck::params::OptionsMap const &options,
                   bdrck::params::FlagsMap const &,
                   bdrck::params::ArgumentsMap const &)
{
	auto keyIt = options.find("key");
	auto setIt = options.find("set");

	if(keyIt == options.end())
	{
		if(setIt != options.end())
		{
			std::cout << "Error: a 'key' must be provided when "
			             "setting a configuration value.\n";
			return;
		}

		std::cout << pwm::config::instance().get().DebugString();
		return;
	}

	if(setIt != options.end())
		pwm::config::setFieldFromString(keyIt->second, setIt->second);

	std::cout << keyIt->second << " = "
	          << pwm::config::getFieldAsString(keyIt->second) << "\n";
}

void initCommand(bdrck::params::OptionsMap const &options,
                 bdrck::params::FlagsMap const &,
                 bdrck::params::ArgumentsMap const &)
{
	pwm::repository::Repository repo(getRepositoryPath(options),
	                                 /*create=*/true);
	std::cout << "Initialized repository: "
	          << repo.repository->getWorkDirectoryPath() << "\n";
}

void listCommand(bdrck::params::OptionsMap const &options,
                 bdrck::params::FlagsMap const &,
                 bdrck::params::ArgumentsMap const &arguments)
{
	pwm::repository::Repository repo(getRepositoryPath(options),
	                                 /*create=*/false);
	pwm::repository::Path path(arguments.find("path")->second.front(),
	                           repo);

	pwm::repository::list(repo, path,
	                      [](pwm::repository::Path const &p) -> bool {
		                      std::cout << p.getRelativePath() << "\n";
		                      return true;
		              });
}

void passwordCommand(bdrck::params::OptionsMap const &options,
                     bdrck::params::FlagsMap const &flags,
                     bdrck::params::ArgumentsMap const &arguments)
{
	pwm::repository::Repository repo(getRepositoryPath(options),
	                                 /*create=*/false);
	pwm::repository::Path path(arguments.find("path")->second.front(),
	                           repo);

	auto setIt = flags.find("set");
	auto keyIt = options.find("key");

	if(setIt->second && keyIt == options.end())
	{
		// The user wants to set the password, but no key file was
		// given, so prompt for the password interactively.

		std::string password = pwm::util::passwordPrompt(
		        "Password: ", /*confirm=*/true);
		pwm::repository::write(
		        repo, path,
		        reinterpret_cast<uint8_t const *>(password.data()),
		        password.length());
	}
	else if(keyIt != options.end())
	{
		// The user wants to set the password using a key file.

		std::ifstream in(keyIt->second,
		                 std::ios_base::in | std::ios_base::binary);
		if(!in.is_open())
		{
			std::ostringstream oss;
			oss << "Failed opening key file '" << keyIt->second
			    << "' for reading.";
			throw std::runtime_error(oss.str());
		}
		pwm::repository::write(repo, path, in);
	}
	else
	{
		// The user wants to retrieve the password, instead of set it.

		std::cout << pwm::repository::read(repo, path) << "\n";
	}
}

#ifdef PWM_DEBUG
void clipboardCommand(bdrck::params::OptionsMap const &options,
                      bdrck::params::FlagsMap const &,
                      bdrck::params::ArgumentsMap const &)
{
	if(options.find("set") != options.end())
	{
		std::string set = options.find("set")->second;
		std::cout << "Set: '" << set << "'\n";
		pwm::util::clipboard::setClipboardContents(
		        pwm::util::clipboard::ClipboardType::Clipboard, set);
	}
	std::cout << pwm::util::clipboard::getClipboardContents(
	                     pwm::util::clipboard::ClipboardType::Clipboard)
	          << "\n";
}
#endif

const std::initializer_list<bdrck::params::Option> CONFIG_COMMAND_OPTIONS{
        bdrck::params::Option::optional("set", "Set the key to this new value.",
                                        's'),
        bdrck::params::Option::optional("key", "The specific key to view/set.",
                                        'k')};

const std::initializer_list<bdrck::params::Option> INIT_COMMAND_OPTIONS{
        bdrck::params::Option::optional(
                "repository", "The path to the repository to initialize.",
                'r')};

const std::initializer_list<bdrck::params::Option> LIST_COMMAND_OPTIONS{
        bdrck::params::Option::optional(
                "repository", "The path to the repository to examine.", 'r')};

const std::vector<bdrck::params::Argument> LIST_COMMAND_ARGUMENTS{
        bdrck::params::Argument(
                "path", "The path to list, relative to the repository's root.",
                "/")};

const std::initializer_list<bdrck::params::Option> PASSWORD_COMMAND_OPTIONS{
        bdrck::params::Option::optional(
                "repository", "The path to the repository to examine.", 'r'),
        bdrck::params::Option::flag(
                "set", "Set this password using a command-line prompt.", 's'),
        bdrck::params::Option::optional(
                "key", "Set this password using a key file.", 'k')};

const std::vector<bdrck::params::Argument> PASSWORD_COMMAND_ARGUMENTS{
        bdrck::params::Argument("path",
                                "The path of the password to get or set.")};

#ifdef PWM_DEBUG
const std::initializer_list<bdrck::params::Option> CLIPBOARD_COMMAND_OPTIONS{
        bdrck::params::Option::optional(
                "set", "Set the clipboard contents to this value.", 's')};
#endif

const std::set<bdrck::params::Command> PWM_COMMANDS = {
        bdrck::params::Command("config", "Get or set a configuration value",
                               configCommand, CONFIG_COMMAND_OPTIONS),
        bdrck::params::Command("init", "Initialize a new pwm repository",
                               initCommand, INIT_COMMAND_OPTIONS),
        bdrck::params::Command(
                "ls", "List passwords stored in a pwm repository", listCommand,
                LIST_COMMAND_OPTIONS, LIST_COMMAND_ARGUMENTS),
        bdrck::params::Command("pw",
                               "Get or set a password from a pwm repository",
                               passwordCommand, PASSWORD_COMMAND_OPTIONS,
                               PASSWORD_COMMAND_ARGUMENTS)
#ifdef PWM_DEBUG
                ,
        bdrck::params::Command("clipboard", "Access clipboard data",
                               clipboardCommand, CLIPBOARD_COMMAND_OPTIONS)
#endif
};
}

int main(int argc, char **argv)
{
#ifdef PWM_USE_CLIPBOARD
	gtk_init(nullptr, nullptr);
#endif

	bdrck::git::LibraryInstance gitLibrary;
	pwm::config::ConfigurationInstance configInstance;
	return bdrck::params::parseAndExecuteCommand(argc, argv, PWM_COMMANDS);
}