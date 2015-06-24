set(pwmc_SOURCES

	args/Argument.cpp
	args/Argument.hpp
	args/Command.cpp
	args/Command.hpp
	args/parseAndExecuteCommand.cpp
	args/parseAndExecuteCommand.hpp
	args/Option.cpp
	args/Option.hpp

	config/Configuration.cpp
	config/Configuration.hpp
	config/ConfigurationRadixTree.cpp
	config/ConfigurationRadixTree.hpp
	config/Key.cpp
	config/Key.hpp

	fs/Util.cpp
	fs/Util.hpp

	git/checkReturn.cpp
	git/checkReturn.hpp
	git/Library.cpp
	git/Library.hpp
	git/Repository.cpp
	git/Repository.hpp
	git/Wrapper.hpp

	util/ScopeExit.cpp
	util/ScopeExit.hpp
	util/String.cpp
	util/String.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
