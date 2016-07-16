set(pwmc_SOURCES

	config/Configuration.cpp
	config/Configuration.hpp

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

	repository/EncryptionHeader.cpp
	repository/EncryptionHeader.hpp
	repository/IO.cpp
	repository/IO.hpp
	repository/Path.cpp
	repository/Path.hpp
	repository/Repository.cpp
	repository/Repository.hpp

	util/Clipboard.cpp
	util/Clipboard.hpp
	util/MemoryFile.cpp
	util/MemoryFile.hpp
	util/OutputSuppressor.cpp
	util/OutputSuppressor.hpp
	util/passwordPrompt.cpp
	util/passwordPrompt.hpp
	util/printBytesAsHex.cpp
	util/printBytesAsHex.hpp

)

add_definitions(-DPROTOBUF_INLINE_NOT_IN_HEADERS=0)
file(GLOB proto_FILES "${CMAKE_CURRENT_SOURCE_DIR}/proto/*.proto")
PROTOBUF_GENERATE_CPP(proto_SOURCES proto_HEADERS ${proto_FILES})

add_library(pwmc STATIC ${pwmc_SOURCES} ${proto_SOURCES} ${proto_HEADERS})
target_link_libraries(pwmc ${pwm_LIBRARIES})
