set(pwm-test_SOURCES

	pwm-test.cpp

)

add_executable(pwm-test ${pwm-test_SOURCES})
target_link_libraries(pwm-test ${pwm_LIBRARIES} pwmc)
add_test(pwm-test pwm-test)
