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
	crypto/decrypt.cpp
	crypto/decrypt.hpp
	crypto/encrypt.cpp
	crypto/encrypt.hpp
	crypto/generatePassword.cpp
	crypto/generatePassword.hpp
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

	repository/EncryptionHeader.cpp
	repository/EncryptionHeader.hpp
	repository/Path.cpp
	repository/Path.hpp

	util/Base64.cpp
	util/Base64.hpp
	util/Clipboard.cpp
	util/Clipboard.hpp
	util/MemoryFile.cpp
	util/MemoryFile.hpp
	util/OutputSuppressor.cpp
	util/OutputSuppressor.hpp
	util/printBytesAsHex.cpp
	util/printBytesAsHex.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
target_link_libraries(pwmc ${pwm_LIBRARIES})
