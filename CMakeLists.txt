cmake_minimum_required(VERSION 3.0.2)
project(pwm)

set(CMAKE_COLOR_MAKEFILE ON)
set(CMAKE_VERBOSE_MAKEFILE OFF)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/3rdparty/cmu/cmake)

# Note which type of build we're performing.

if(NOT CMAKE_BUILD_TYPE)
	message(STATUS "No CMAKE_BUILD_TYPE specified; defaulting to Release.")
	set(CMAKE_BUILD_TYPE "Release")
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_LOWER)

# Define various CMake options.

option(USE_CLIPBOARD "Enable clipboard support" ON)

# Find our dependencies.

set(CMAKE_AUTOMOC ON)

find_package(Boost REQUIRED)
find_package(Protobuf REQUIRED)
find_package(Libgit2 REQUIRED)
find_package(Yajl REQUIRED)
find_package(Libgcrypt REQUIRED) # Must be 1.6.0 or newer.

if(USE_CLIPBOARD)
	find_package(GTK3 REQUIRED)
	include_directories(SYSTEM ${GTK3_INCLUDE_DIRS})
endif()

# Setup our compile flags.

include(${PROJECT_SOURCE_DIR}/3rdparty/cmu/cmake/SetFlags.cmake)
cmuSetCompileFlags()

if(CMAKE_BUILD_TYPE_LOWER MATCHES debug)
	add_definitions(-DPWM_DEBUG)
endif()

if(USE_CLIPBOARD)
	add_definitions(-DPWM_USE_CLIPBOARD)
endif()

include_directories(src)

include_directories(SYSTEM

	3rdparty/include
	${Boost_INCLUDE_DIRS}
	${PROTOBUF_INCLUDE_DIRS}
	${CMAKE_BINARY_DIR}/src/pwmc

)

# Define the libraries to link against.

set(pwm_LIBRARIES

	${PROTOBUF_LIBRARIES}
	${LIBGIT2_LIBRARIES}
	${YAJL_LIBRARIES}
	${LIBGCRYPT_LIBRARIES}
	bdrck-algorithm
	bdrck-config
	bdrck-fs
	bdrck-git
	bdrck-params
	bdrck-util

)

if(USE_CLIPBOARD)
	include_directories(${GTK3_INCLUDE_DIRS})

	set(pwm_LIBRARIES

		${pwm_LIBRARIES}
		${GTK3_LIBRARIES}

	)
endif()

# Add our targets.

enable_testing()

add_subdirectory(3rdparty/bdrck)
add_subdirectory(src/pwmc)
add_subdirectory(src/pwm)
add_subdirectory(src/test)
