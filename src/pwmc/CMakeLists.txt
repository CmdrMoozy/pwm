set(pwmc_SOURCES

	git/checkReturn.cpp
	git/checkReturn.hpp
	git/Library.cpp
	git/Library.hpp
	git/Repository.cpp
	git/Repository.hpp
	git/Wrapper.hpp

)

add_library(pwmc STATIC ${pwmc_SOURCES})
