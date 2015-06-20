# Locate libgit2. Once completed, this will define the following:
#
# LIBGIT2_FOUND - True if libgit2 was found
# LIBGIT2_LIBRARIES - The libraries which should be linked for libgit2
# LIBGIT2_INCLUDE_DIRS - The include directories for libgit2
# LIBGIT2_DEFINITIONS - The preprocessor definitions required for libgit2

# Use pkg-config to get the directories and then use these values
# in the FIND_PATH() and FIND_LIBRARY() calls
find_package(PkgConfig)
pkg_check_modules(PKG_GIT2 QUIET git2)

set(LIBGIT2_DEFINITIONS ${PKG_GIT2_CFLAGS_OTHER})

find_path(LIBGIT2_INCLUDE_DIR
    NAMES
        git2.h
    HINTS
        ${PKG_GIT2_INCLUDE_DIRS}
)
find_library(LIBGIT2_LIBRARY
    NAMES
        git2
    HINTS
        ${PKG_GIT2_LIBRARY_DIRS}
)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Libgit2
    FOUND_VAR
        LIBGIT2_FOUND
    REQUIRED_VARS
        LIBGIT2_LIBRARY
        LIBGIT2_INCLUDE_DIR
)

mark_as_advanced(LIBGIT2_LIBRARY LIBGIT2_INCLUDE_DIR)

set(LIBGIT2_LIBRARIES ${LIBGIT2_LIBRARY})
set(LIBGIT2_INCLUDE_DIRS ${LIBGIT2_INCLUDE_DIR})
