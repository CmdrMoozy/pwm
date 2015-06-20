set(pwm_SOURCES

	pwm.cpp

)

add_executable(pwm ${pwm_SOURCES})
target_link_libraries(pwm ${pwm_LIBRARIES} pwmc)
