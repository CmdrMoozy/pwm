set(pwm-test_SOURCES

	pwm-test.cpp

	config/ConfigurationTest.cpp

	crypto/EncryptDecryptTest.cpp
	crypto/GeneratePasswordTest.cpp
	crypto/KeyTest.cpp

	repository/EncryptionHeaderTest.cpp

	util/Base64Test.cpp
	util/ClipboardTest.cpp
	util/MemoryFileTest.cpp

)

add_executable(pwm-test ${pwm-test_SOURCES})
target_link_libraries(pwm-test ${pwm_LIBRARIES} pwmc)
add_test(pwm-test pwm-test)
