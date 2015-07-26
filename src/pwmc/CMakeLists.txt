set(pwmc_SOURCES

	config/Configuration.cpp
	config/Configuration.hpp
	config/ConfigurationRadixTree.cpp
	config/ConfigurationRadixTree.hpp
	config/deserializeConfiguration.cpp
	config/deserializeConfiguration.hpp
	config/Key.cpp
	config/Key.hpp
	config/serializeConfiguration.cpp
	config/serializeConfiguration.hpp

	crypto/checkReturn.cpp
	crypto/checkReturn.hpp
	crypto/Key.cpp
	crypto/Key.hpp
	crypto/Util.cpp
	crypto/Util.hpp

	fs/Util.cpp
	fs/Util.hpp

	git/checkReturn.cpp
	git/checkReturn.hpp
	git/Library.cpp
	git/Library.hpp
	git/Repository.cpp
	git/Repository.hpp
	git/Wrapper.hpp

	params/Argument.cpp
	params/Argument.hpp
	params/Command.cpp
	params/Command.hpp
	params/Option.cpp
	params/Option.hpp
	params/parseAndExecuteCommand.cpp
	params/parseAndExecuteCommand.hpp
	params/ProgramParameters.cpp
	params/ProgramParameters.hpp

	params/detail/parseArguments.cpp
	params/detail/parseArguments.hpp
	params/detail/parseCommand.cpp
	params/detail/parseCommand.hpp
	params/detail/parseOptions.cpp
	params/detail/parseOptions.hpp

	util/Base64.cpp
	util/Base64.hpp
	util/Clipboard.cpp
	util/Clipboard.hpp
	util/OutputSuppressor.cpp
	util/OutputSuppressor.hpp
	util/printBytesAsHex.cpp
	util/printBytesAsHex.hpp
	util/ScopeExit.cpp
	util/ScopeExit.hpp
	util/String.cpp
	util/String.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
