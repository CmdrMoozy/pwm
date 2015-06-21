cmake_minimum_required(VERSION 3.0.2)
project(pwm)

set(CMAKE_COLOR_MAKEFILE ON)
set(CMAKE_VERBOSE_MAKEFILE OFF)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

# Note which type of build we're performing.

if(NOT CMAKE_BUILD_TYPE)
	message(STATUS "No CMAKE_BUILD_TYPE specified; defaulting to Release.")
	set(CMAKE_BUILD_TYPE "Release")
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LOWER)

# Find our dependencies.

set(CMAKE_AUTOMOC ON)

find_package(Libgit2 REQUIRED)
find_package(Yajl REQUIRED)

# Setup our compile flags.

include(${PROJECT_SOURCE_DIR}/cmake/SetFlags.cmake)
pwmSetCompileFlags()

if(CMAKE_BUILD_TYPE_LOWER MATCHES debug)
	add_definitions(-DPWM_DEBUG)
endif()

include_directories(3rdparty/include src)

# Define the libraries to link against.

set(pwm_LIBRARIES

	${LIBGIT2_LIBRARIES}
	${YAJL_LIBRARIES}

)

# Add our targets.

enable_testing()

add_subdirectory(src/pwmc)
add_subdirectory(src/pwm)

if(CMAKE_BUILD_TYPE_LOWER MATCHES debug)
	add_subdirectory(src/test)
endif()
