set(pwm-test_SOURCES

	pwm-test.cpp

	crypto/EncryptDecryptTest.cpp
	crypto/GeneratePasswordTest.cpp
	crypto/KeyTest.cpp
	crypto/PaddingTest.cpp

	repository/EncryptionHeaderTest.cpp

	util/ClipboardTest.cpp
	util/MemoryFileTest.cpp

)

add_executable(pwm-test ${pwm-test_SOURCES})
target_link_libraries(pwm-test ${pwm_LIBRARIES} pwmc)
add_test(pwm-test pwm-test)
