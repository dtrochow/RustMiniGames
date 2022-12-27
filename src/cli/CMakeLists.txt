set(TARGET_NAME rumiga)
add_executable(${TARGET_NAME})

target_sources(${TARGET_NAME} PRIVATE 
    cli_main.cpp
)

target_link_libraries(${TARGET_NAME} PRIVATE
    test-lib
    argagg
)

target_compile_features(${TARGET_NAME} PUBLIC 
    cxx_std_17
)

# if(WIN32)
#     target_link_libraries(helloworld test-lib ws2_32 userenv)
# elseif(CMAKE_SYSTEM_NAME MATCHES "Darwin")
#     target_link_libraries(helloworld test-lib resolv)
# else()
#     target_link_libraries(helloworld test-lib pthread dl m)
# endif()
