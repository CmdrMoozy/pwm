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

#include <cstdlib>
#include <initializer_list>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#ifdef PWM_USE_CLIPBOARD
#include <gtk/gtk.h>
#endif

#include "pwmc/config/Configuration.hpp"
#include "pwmc/git/Library.hpp"
#include "pwmc/git/Repository.hpp"
#include "pwmc/params/Argument.hpp"
#include "pwmc/params/Command.hpp"
#include "pwmc/params/Option.hpp"
#include "pwmc/params/parseAndExecuteCommand.hpp"

#ifdef PWM_DEBUG
#include "pwmc/util/Clipboard.hpp"
#endif

namespace
{
void configCommand(pwm::params::OptionsMap const &options,
                   pwm::params::FlagsMap const &,
                   pwm::params::ArgumentsMap const &arguments)
{
	pwm::config::Key key(*arguments.find("key")->second.begin());

	if(options.find("set") != options.end())
	{
		pwm::config::Configuration::getInstance().set(
		        key, options.find("set")->second);
	}

	std::cout << pwm::config::Key(key) << " = "
	          << pwm::config::Configuration::getInstance().get(key) << "\n";
}

void initCommand(pwm::params::OptionsMap const &, pwm::params::FlagsMap const &,
                 pwm::params::ArgumentsMap const &arguments)
{
	std::string repoPath = *arguments.find("repository")->second.begin();
	if(repoPath == pwm::config::getUseConfigDefaultArgument())
	{
		repoPath = pwm::config::Configuration::getInstance().getOr(
		        pwm::config::getConfigurationKey(
		                pwm::config::ConfigurationValue::
		                        RepositoryDefaultPath),
		        "");
	}

	if(repoPath.empty())
	{
		std::ostringstream oss;
		oss << "No repository path specified. Try the 'repository' "
		       "command option, or setting the '"
		    << pwm::config::getConfigurationKey(
		               pwm::config::ConfigurationValue::
		                       RepositoryDefaultPath)
		    << "' configuration key.";
		throw std::runtime_error(oss.str());
	}

	pwm::git::Repository repo(
	        repoPath, pwm::git::RepositoryCreateMode::CreateNormal, false);
}

void listCommand(pwm::params::OptionsMap const &, pwm::params::FlagsMap const &,
                 pwm::params::ArgumentsMap const &)
{
}

#ifdef PWM_DEBUG
void clipboardCommand(pwm::params::OptionsMap const &options,
                      pwm::params::FlagsMap const &,
                      pwm::params::ArgumentsMap const &)
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

const std::initializer_list<pwm::params::Option> CONFIG_COMMAND_OPTIONS{
        pwm::params::Option::optional("set", "Set the key to this new value.",
                                      's')};

const std::vector<pwm::params::Argument> CONFIG_COMMAND_ARGUMENTS{
        pwm::params::Argument("key", "The configuration key to get or set.")};

const std::vector<pwm::params::Argument> INIT_COMMAND_ARGUMENTS{
        pwm::params::Argument("repository",
                              "The path to the repository to initialize.",
                              pwm::config::getUseConfigDefaultArgument())};

const std::initializer_list<pwm::params::Option> LIST_COMMAND_OPTIONS{
        pwm::params::Option::optional(
                "repository", "The path to the repository to examine.", 'r')};

const std::vector<pwm::params::Argument> LIST_COMMAND_ARGUMENTS{
        pwm::params::Argument(
                "path", "The path to list, relative to the repository's root.",
                "/")};

#ifdef PWM_DEBUG
const std::initializer_list<pwm::params::Option> CLIPBOARD_COMMAND_OPTIONS{
        pwm::params::Option::optional(
                "set", "Set the clipboard contents to this value.", 's')};
#endif

const std::set<pwm::params::Command> PWM_COMMANDS = {
        pwm::params::Command("config", "Get or set a configuration value",
                             configCommand, CONFIG_COMMAND_OPTIONS,
                             CONFIG_COMMAND_ARGUMENTS),
        pwm::params::Command("init", "Initialize a new pwm repository",
                             initCommand, {}, INIT_COMMAND_ARGUMENTS),
        pwm::params::Command("ls", "List passwords stored in a pwm repository",
                             listCommand, LIST_COMMAND_OPTIONS,
                             LIST_COMMAND_ARGUMENTS)
#ifdef PWM_DEBUG
                ,
        pwm::params::Command("clipboard", "Access clipboard data",
                             clipboardCommand, CLIPBOARD_COMMAND_OPTIONS)
#endif
};
}

int main(int argc, char **argv)
{
#ifdef PWM_USE_CLIPBOARD
	gtk_init(nullptr, nullptr);
#endif

	pwm::git::LibraryInstance gitLibrary;
	pwm::config::ConfigurationInstance configInstance;
	return pwm::params::parseAndExecuteCommand(argc, argv, PWM_COMMANDS);
}
