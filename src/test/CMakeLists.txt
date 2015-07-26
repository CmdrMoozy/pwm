set(pwm-test_SOURCES

	pwm-test.cpp

	config/ConfigurationTest.cpp

	crypto/KeyTest.cpp

	params/CommandTest.cpp
	params/OptionTest.cpp

	params/detail/ParseArgumentsTest.cpp
	params/detail/ParseCommandTest.cpp
	params/detail/ParseOptionsTest.cpp

	util/ClipboardTest.cpp
	util/StringTest.cpp

)

add_executable(pwm-test ${pwm-test_SOURCES})
target_link_libraries(pwm-test ${pwm_LIBRARIES} pwmc)
add_test(pwm-test pwm-test)
