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
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#ifdef PWM_USE_CLIPBOARD
#include <gtk/gtk.h>
#endif

#include "pwmc/args/Argument.hpp"
#include "pwmc/args/Command.hpp"
#include "pwmc/args/Option.hpp"
#include "pwmc/args/parseAndExecuteCommand.hpp"
#include "pwmc/config/Configuration.hpp"
#include "pwmc/git/Library.hpp"
#include "pwmc/git/Repository.hpp"

#ifdef PWM_DEBUG
#include "pwmc/util/Clipboard.hpp"
#endif

namespace
{
void configCommand(const pwm::args::OptionsMap &options,
                   const pwm::args::FlagsMap &,
                   const pwm::args::ArgumentsMap &arguments)
{
	pwm::config::Key key(*arguments.find("key")->second.begin());
	auto valIt = options.find("set");
	if(valIt != options.end())
	{
		pwm::config::Configuration::getInstance().set(key,
		                                              valIt->second);
	}

	std::cout << pwm::config::Key(key) << " = "
	          << pwm::config::Configuration::getInstance().get(key) << "\n";
}

void initCommand(const pwm::args::OptionsMap &options,
                 const pwm::args::FlagsMap &, const pwm::args::ArgumentsMap &)
{
	std::string repoPath = options.find("repository")->second;
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

#ifdef PWM_DEBUG
void clipboardCommand(const pwm::args::OptionsMap &options,
                      const pwm::args::FlagsMap &,
                      const pwm::args::ArgumentsMap &)
{
	auto setIt = options.find("set");
	if(setIt != options.end())
	{
		std::cout << "Set: '" << setIt->second << "'\n";
		pwm::util::clipboard::setClipboardContents(
		        pwm::util::clipboard::ClipboardType::Clipboard,
		        setIt->second);
	}
	std::cout << pwm::util::clipboard::getClipboardContents(
	                     pwm::util::clipboard::ClipboardType::Clipboard)
	          << "\n";
}
#endif

const std::vector<pwm::args::Option> CONFIG_COMMAND_OPTIONS = {
        pwm::args::Option("set", "Set the key to this new value.", 's', true)};

const std::vector<pwm::args::Argument> CONFIG_COMMAND_ARGUMENTS = {
        pwm::args::Argument("key", "The configuration key to get or set."),
};

const std::vector<pwm::args::Option> INIT_COMMAND_OPTIONS = {pwm::args::Option(
        "repository", "The path to the repository to initialize.", 'r', false,
        pwm::config::getUseConfigDefaultArgument())};

#ifdef PWM_DEBUG
const std::vector<pwm::args::Option> CLIPBOARD_COMMAND_OPTIONS = {
        pwm::args::Option("set", "Set the clipboard contents to this value.",
                          's', true)};
#endif

const std::vector<pwm::args::Command> PWM_COMMANDS = {
        pwm::args::Command("config", "Get or set a configuration value",
                           configCommand, CONFIG_COMMAND_OPTIONS,
                           CONFIG_COMMAND_ARGUMENTS),
        pwm::args::Command("init", "Initialize a new pwm repository",
                           initCommand, INIT_COMMAND_OPTIONS, {})
#ifdef PWM_DEBUG
                ,
        pwm::args::Command("clipboard", "Access clipboard data",
                           clipboardCommand, CLIPBOARD_COMMAND_OPTIONS, {})
#endif
};
}

int main(int argc, char *const *argv)
{
#ifdef PWM_USE_CLIPBOARD
	gtk_init(nullptr, nullptr);
#endif

	pwm::git::LibraryInstance gitLibrary;
	pwm::config::ConfigurationInstance configInstance;
	int ret = pwm::args::parseAndExecuteCommand(argc, argv, PWM_COMMANDS);
	return ret;
}
