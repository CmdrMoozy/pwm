# Locate libgcrypt. Once found, this will define:
#
#   LIBGCRYPT_FOUND
#   LIBGCRYPT_LIBRARIES

find_path(LIBGCRYPT_INCLUDE_DIR NAMES gcrypt.h)
find_library(LIBGCRYPT_LIBRARIES NAMES gcrypt)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Libgcrypt
	FOUND_VAR
		LIBGCRYPT_FOUND
	REQUIRED_VARS
		LIBGCRYPT_LIBRARIES
		LIBGCRYPT_INCLUDE_DIR
)
