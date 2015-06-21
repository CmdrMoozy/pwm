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

	fs/Util.cpp
	fs/Util.hpp

	git/checkReturn.cpp
	git/checkReturn.hpp
	git/Library.cpp
	git/Library.hpp
	git/Repository.cpp
	git/Repository.hpp
	git/Wrapper.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
