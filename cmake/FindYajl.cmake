# Locate yajl. Once completed, this will define the following:
#
# YAJL_FOUND - True if yajl was found
# YAJL_LIBRARIES - The libraries which should be linked for yajl.
# YAJL_INCLUDE_DIRS - The include directories for yajl

find_path(YAJL_INCLUDE_DIR NAMES yajl/yajl_common.h)
find_library(YAJL_LIBRARY NAMES yajl)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Yajl
	FOUND_VAR
		YAJL_FOUND
	REQUIRED_VARS
		YAJL_LIBRARY
		YAJL_INCLUDE_DIR
)

mark_as_advanced(YAJL_LIBRARY YAJL_INCLUDE_DIR)

set(YAJL_LIBRARIES ${YAJL_LIBRARY})
set(YAJL_INCLUDE_DIRS ${YAJL_INCLUDE_DIR})
