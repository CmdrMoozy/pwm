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
	config/deserializeConfiguration.cpp
	config/deserializeConfiguration.hpp
	config/Key.cpp
	config/Key.hpp
	config/serializeConfiguration.cpp
	config/serializeConfiguration.hpp

	crypto/checkReturn.cpp
	crypto/checkReturn.hpp
	crypto/Passphrase.cpp
	crypto/Passphrase.hpp
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
	params/ProgramParameters.cpp
	params/ProgramParameters.hpp

	util/Clipboard.cpp
	util/Clipboard.hpp
	util/OutputSuppressor.cpp
	util/OutputSuppressor.hpp
	util/ScopeExit.cpp
	util/ScopeExit.hpp
	util/String.cpp
	util/String.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
